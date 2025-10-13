use meowtonin::{ByondResult, ByondValue, byond_fn};
use hierarchical_hash_wheel_timer::*;
use std::time::Duration;
use uuid::Uuid;
use crate::timer::TimerRef;



const TIMER_RESCHEDULE: &str = "TIMER_RESCHEDULE";
const TIMER_CANCEL: &str = "TIMER_CANCEL";
const ERROR_CALLBACK_PROC: &str = "rt_timer_error";

pub enum TimerType {
    RealTime,
    ByondTick,
}

pub fn get_uuid(utype: TimerType) -> Uuid {

    // this is basically the same as what uuid crate itself does when it creates v4 uuids using the "fast-rng" feature
    let mut buf: [u8; 16] = rand::random();

    // but we mangle one byte, so that we can tell where the id originated in when we cancel. concat_bytes! is nightly :(
    match utype {
        TimerType::RealTime => buf[0] = 0,   // 00
        TimerType::ByondTick => buf[0] = 189, // BD
    }
    Uuid::new_v8(buf)
}

pub trait TimerTypable {
    fn timertype(&self) -> TimerType;
}

impl TimerTypable for Uuid {
    fn timertype(&self) -> TimerType {
        match self.as_bytes()[0] {
            189 => TimerType::ByondTick,
            _ => TimerType::RealTime
        }
    }
}

#[byond_fn]
pub fn cancel_timer(strid: String) {
    if let Ok(id) = Uuid::parse_str(&strid) {
        match id.timertype() {
            TimerType::ByondTick => crate::byondtimers::cancel_timer(id),
            TimerType::RealTime => crate::realtimers::cancel_timer(id),
        }
    }
}

pub fn schedule_oneshot_timer(timers: &mut TimerRef<Uuid, OneShotClosureState<Uuid>, PeriodicClosureState<Uuid>>, id: Uuid, delay: Duration, owning_obj: ByondValue, proc_path: ByondValue, proc_args: ByondValue) {
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
}

pub fn schedule_periodic_timer(timers: &mut TimerRef<Uuid, OneShotClosureState<Uuid>, PeriodicClosureState<Uuid>>, id: Uuid, delay: Duration, period: Duration, owning_obj: ByondValue, proc_path: ByondValue, proc_args: ByondValue) {
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
}

pub fn should_reschedule(str_in: Option<String>) -> TimerReturn<()> {
    match str_in.as_deref() {
        Some(TIMER_RESCHEDULE) => TimerReturn::Reschedule(()),
        Some(TIMER_CANCEL) => TimerReturn::Cancel,
        _ => TimerReturn::Reschedule(()),
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

