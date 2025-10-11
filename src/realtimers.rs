use crate::core::*;
use crate::timer::*;
use hierarchical_hash_wheel_timer::*;
use meowtonin::{ByondError, ByondResult, ByondValue, byond_fn};
use std::sync::{LazyLock, Mutex};
use std::time::Duration;
use uuid::Uuid;

type TimerCoreType = TimerWithThread<Uuid, OneShotClosureState<Uuid>, PeriodicClosureState<Uuid>>;
type TimerRefType = TimerRef<Uuid, OneShotClosureState<Uuid>, PeriodicClosureState<Uuid>>;

pub static TIMER_CORE: LazyLock<TimerCoreType> = LazyLock::new(TimerWithThread::for_uuid_closures);
pub static TIMER: LazyLock<Mutex<TimerRefType>> =
    LazyLock::new(|| Mutex::new(TIMER_CORE.timer_ref()));

#[byond_fn]
pub fn schedule_once(
    delay: u64,
    owning_obj: ByondValue,
    proc_path: ByondValue,
    proc_args: ByondValue,
) -> ByondResult<String> {
    let id = get_uuid(TimerType::RealTime);
    let delay = Duration::from_millis(delay);

    if owning_obj.is_null() || proc_path.is_null() {
        return Err(ByondError::InvalidProc);
    }

    proc_args.inc_ref();

    let mut timers = TIMER.lock().unwrap();

    schedule_oneshot_timer(&mut timers, id, delay, owning_obj, proc_path, proc_args);

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
    let id = get_uuid(TimerType::RealTime);
    let delay = Duration::from_millis(delay);
    let period = Duration::from_millis(period);

    if owning_obj.is_null() || proc_path.is_null() {
        return Err(ByondError::InvalidProc);
    }

    proc_args.inc_ref();
    let mut timers = TIMER.lock().unwrap();

    schedule_periodic_timer(&mut timers, id, delay, period, owning_obj, proc_path, proc_args);

    Ok(id.to_string())
}


pub fn cancel_timer(id: Uuid) {
    TIMER.lock().unwrap().cancel(&id)
}
