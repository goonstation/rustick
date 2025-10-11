use crate::core::*;
use crate::timer::*;
use hierarchical_hash_wheel_timer::*;
use lazy_static::lazy_static;
use meowtonin::{ByondError, ByondResult, ByondValue, byond_fn};
use std::sync::Mutex;
use std::time::Duration;
use uuid::Uuid;

lazy_static! {
    // real time timers, ticking in their own thread
    // this is here because i'm lazy
    pub static ref BYOND_TIMER_CORE: TimerWithThread<Uuid, OneShotClosureState<Uuid>, PeriodicClosureState<Uuid>> = TimerWithThread::for_uuid_closures_sans_autotick();
    // this is here because it's actually useful
    pub static ref BYOND_TIMER: Mutex<TimerRef<Uuid, OneShotClosureState<Uuid>, PeriodicClosureState<Uuid>>> = Mutex::new(BYOND_TIMER_CORE.timer_ref());

}

#[byond_fn]
pub fn schedule_once_tick(
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

    let mut timers = BYOND_TIMER.lock().unwrap();


    schedule_oneshot_timer(&mut timers, id, delay, owning_obj, proc_path, proc_args);

    Ok(id.to_string())
}

#[byond_fn]
pub fn schedule_periodic_tick(
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
    let mut timers = BYOND_TIMER.lock().unwrap();

    schedule_periodic_timer(&mut timers, id, delay, period, owning_obj, proc_path, proc_args);

    Ok(id.to_string())
}

#[byond_fn]
pub fn cancel_timer_byondtick(strid: String) {
    if let Ok(id) = Uuid::parse_str(&strid) {
        BYOND_TIMER.lock().unwrap().cancel(&id)
    }
}

#[byond_fn]
pub fn tick_byondtick() {
    BYOND_TIMER.lock().expect("tick timer").tick()
}
