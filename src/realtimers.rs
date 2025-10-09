use hierarchical_hash_wheel_timer::thread_timer::*;
use hierarchical_hash_wheel_timer::*;
use lazy_static::lazy_static;
use meowtonin::{ByondError, ByondResult, ByondValue, byond_fn};
use std::sync::Mutex;
use std::time::Duration;
use uuid::Uuid;

const TIMER_RESCHEDULE: &str = "TIMER_RESCHEDULE";
const TIMER_CANCEL: &str = "TIMER_CANCEL";
const ERROR_CALLBACK_PROC: &str = "rt_timer_error";

lazy_static! {
    // real time timers, ticking in their own thread
    // this is here because i'm lazy
    pub static ref TIMER_CORE: TimerWithThread<Uuid, OneShotClosureState<Uuid>, PeriodicClosureState<Uuid>> = TimerWithThread::for_uuid_closures();
    // this is here because it's actually useful
    pub static ref TIMER: Mutex<TimerRef<Uuid, OneShotClosureState<Uuid>, PeriodicClosureState<Uuid>>> = Mutex::new(TIMER_CORE.timer_ref());

}

/*
pub static TIMER_CORE: LazyLock<TimerWithThread<Uuid, OneShotClosureState<Uuid>, PeriodicClosureState<Uuid>>> = LazyLock::new(|| {
    TimerWithThread::for_uuid_closures()
});
pub static TIMER: LazyLock<Mutex<TimerRef<Uuid, OneShotClosureState<Uuid>, PeriodicClosureState<Uuid>>>> = LazyLock::new(|| {
    Mutex::new(TIMER_CORE.timer_ref())
});
*/

#[byond_fn]
pub fn schedule_once(
    delay: u64,
    owning_obj: ByondValue,
    proc_path: ByondValue,
    proc_args: ByondValue,
) -> ByondResult<String> {
    let id = Uuid::new_v4();
    let delay = Duration::from_millis(delay);

    if owning_obj.is_null() || proc_path.is_null() {
        return Err(ByondError::InvalidProc);
    }

    proc_args.inc_ref();

    let mut timers = TIMER.lock().unwrap();

    if can_have_procs(&owning_obj) {
        // Meowtonin catches panics and converts them to runtimes, but if the closure here panics the timing thread dies and you won't find out (subsequent calls might panic in the meowtonin thread to let you know tho)
        owning_obj.inc_ref();
        timers.schedule_action_once(id, delay, move |_timer_id| {
            if let Err(e) = call_owned_proc(&owning_obj, &proc_path, &proc_args) {
                scream_at_byond(e.to_string());
            }
            proc_args.dec_ref();
            owning_obj.dec_ref();
        });
    } else {
        timers.schedule_action_once(id, delay, move |_timer_id| {
            if let Err(e) = call_global_proc(&proc_path, &proc_args) {
                scream_at_byond(e.to_string());
            }
            proc_args.dec_ref();
        });
    }

    Ok(id.to_string())
}

#[byond_fn]
pub fn schedule_periodic(
    delay: u64,
    period: u64,
    owning_obj: ByondValue,
    proc_path: ByondValue,
    proc_args: ByondValue,
) -> ByondResult<String> {
    let id = Uuid::new_v4();
    let delay = Duration::from_millis(delay);
    let period = Duration::from_millis(period);

    if owning_obj.is_null() || proc_path.is_null() {
        return Err(ByondError::InvalidProc);
    }

    proc_args.inc_ref();
    let mut timers = TIMER.lock().unwrap();

    if can_have_procs(&owning_obj) {
        // Meowtonin catches panics and converts them to runtimes, but if the closure here panics the timing thread dies and you won't find out (subsequent calls might panic in the meowtonin thread to let you know tho)
        timers.schedule_action_periodic(id, delay, period, move |_timer_id| match call_owned_proc(
            &owning_obj,
            &proc_path,
            &proc_args,
        ) {
            Ok(ret) => {
                let res = should_reschedule(ret);
                if res == TimerReturn::Cancel {
                    owning_obj.dec_ref();
                    proc_args.dec_ref();
                }
                res
            }
            Err(e) => {
                scream_at_byond(e.to_string());
                owning_obj.dec_ref();
                proc_args.dec_ref();
                TimerReturn::Cancel
            }
        });
    } else {
        timers.schedule_action_periodic(
            id,
            delay,
            period,
            move |_timer_id| match call_global_proc(&proc_path, &proc_args) {
                Ok(ret) => {
                    let res = should_reschedule(ret);
                    if res == TimerReturn::Cancel {
                        proc_args.dec_ref();
                    }
                    res
                }
                Err(e) => {
                    scream_at_byond(e.to_string());
                    proc_args.dec_ref();
                    TimerReturn::Cancel
                }
            },
        );
    }

    Ok(id.to_string())
}

#[byond_fn]
pub fn cancel_timer(strid: String) {
    if let Ok(id) = Uuid::parse_str(&strid) {
        TIMER.lock().unwrap().cancel(&id)
    }
}

pub fn should_reschedule(str_in: Option<String>) -> TimerReturn<()> {
    match str_in.as_deref() {
        Some(TIMER_RESCHEDULE) => TimerReturn::Reschedule(()),
        Some(TIMER_CANCEL) => TimerReturn::Cancel,
        _ => TimerReturn::Reschedule(()),
    }
}

pub fn should_reschedule_ostring(value_in: ByondResult<Option<String>>) -> TimerReturn<()> {
    match value_in {
        Ok(stri) => match stri.as_deref() {
            Some(TIMER_RESCHEDULE) => TimerReturn::Reschedule(()),
            Some(TIMER_CANCEL) => TimerReturn::Cancel,
            _ => TimerReturn::Reschedule(()),
        },
        Err(_) => TimerReturn::Cancel,
    }
}

pub fn can_have_procs(type_in: &ByondValue) -> bool {
    meowtonin::value::typecheck::ByondValueType::PROC_HAVING_TYPES.contains(&type_in.get_type())
}

pub fn call_global_proc(
    proc_path_bv: &ByondValue,
    proc_args_bv: &ByondValue,
) -> ByondResult<Option<String>> {
    let proc_path = proc_path_bv.get_string()?;
    let proc_args = proc_args_bv.read_list()?;

    meowtonin::call_global::<_, _, _, Option<String>>(proc_path, proc_args)
}

pub fn call_owned_proc(
    proc_owner: &ByondValue,
    proc_path_bv: &ByondValue,
    proc_args_bv: &ByondValue,
) -> ByondResult<Option<String>> {
    let proc_path = proc_path_bv.get_string()?;
    let proc_args = proc_args_bv.read_list()?;

    proc_owner.call::<_, _, _, Option<String>>(proc_path, proc_args)
}

pub fn scream_at_byond(aieee: String) {
    let _ = meowtonin::call_global::<_, _, _, Option<String>>(ERROR_CALLBACK_PROC, [aieee]);
}
