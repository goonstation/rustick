use crate::core::*;
use crate::timer::*;
use hierarchical_hash_wheel_timer::*;
use meowtonin::{ByondError, ByondResult, ByondValue, byond_fn};
use std::sync::{LazyLock, Mutex};
use std::time::Duration;
use uuid::Uuid;

type TimerCoreType = TimerWithThread<Uuid, OneShotClosureState<Uuid>, PeriodicClosureState<Uuid>>;
type TimerRefType = TimerRef<Uuid, OneShotClosureState<Uuid>, PeriodicClosureState<Uuid>>;

pub static TIMER_CORE: LazyLock<Option<TimerCoreType>> =
    LazyLock::new(|| match TimerWithThread::for_uuid_closures() {
        Ok(timer) => Some(timer),
        Err(e) => {
            log_error(format!("failed to start real-time timer thread: {e}"));
            None
        }
    });
pub static TIMER: LazyLock<Mutex<Option<TimerRefType>>> =
    LazyLock::new(|| Mutex::new(TIMER_CORE.as_ref().map(TimerWithThread::timer_ref)));

/// Schedules a one-shot timer based on real-time (milliseconds).
///
/// # Arguments
/// * `delay` - Milliseconds to wait before executing the proc
/// * `owning_obj` - The BYOND object that owns the proc to call
/// * `proc_path` - The path to the proc to call
/// * `proc_args` - Arguments to pass to the proc
///
/// # Returns
/// * A UUID string identifying the timer for cancellation
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

    let mut timers = match TIMER.lock() {
        Ok(timers) => timers,
        Err(e) => {
            log_error(format!("failed to acquire real-time timer lock: {e}"));
            return Err(ByondError::InvalidProc);
        }
    };

    let Some(timers) = timers.as_mut() else {
        log_error("real-time timer thread is unavailable");
        return Err(ByondError::InvalidProc);
    };

    schedule_oneshot_timer(timers, id, delay, owning_obj, proc_path, proc_args);

    Ok(id.to_string())
}

/// Schedules a recurring timer based on real-time (milliseconds).
///
/// # Arguments
/// * `delay` - Milliseconds to wait before first execution
/// * `period` - Milliseconds between recurring executions
/// * `owning_obj` - The BYOND object that owns the proc to call
/// * `proc_path` - The path to the proc to call
/// * `proc_args` - Arguments to pass to the proc
///
/// # Returns
/// * A UUID string identifying the timer for cancellation
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

    let mut timers = match TIMER.lock() {
        Ok(timers) => timers,
        Err(e) => {
            log_error(format!("failed to acquire real-time timer lock: {e}"));
            return Err(ByondError::InvalidProc);
        }
    };

    let Some(timers) = timers.as_mut() else {
        log_error("real-time timer thread is unavailable");
        return Err(ByondError::InvalidProc);
    };

    schedule_periodic_timer(timers, id, delay, period, owning_obj, proc_path, proc_args);

    Ok(id.to_string())
}

/// Cancels a real-time timer based on its UUID.
pub fn cancel_timer(id: Uuid) {
    match TIMER.lock() {
        Ok(mut timers) => match timers.as_mut() {
            Some(timers) => timers.cancel(&id),
            None => log_error("real-time timer thread is unavailable"),
        },
        Err(e) => log_error(format!("failed to acquire real-time timer lock: {e}")),
    }
}
