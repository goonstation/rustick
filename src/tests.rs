use crate::core::{TimerTypable, TimerType, get_uuid, should_reschedule};
use hierarchical_hash_wheel_timer::TimerReturn;
use uuid::Uuid;

#[test]
fn should_reschedule_for_reschedule_marker() {
    let result = should_reschedule(Some("TIMER_RESCHEDULE".to_string()));

    assert_eq!(result, TimerReturn::Reschedule(()));
}

#[test]
fn should_cancel_for_cancel_marker() {
    let result = should_reschedule(Some("TIMER_CANCEL".to_string()));

    assert_eq!(result, TimerReturn::Cancel);
}

#[test]
fn should_reschedule_by_default() {
    let result = should_reschedule(None);

    assert_eq!(result, TimerReturn::Reschedule(()));
}

#[test]
fn uuid_prefix_routes_to_timer_type() {
    let real_time_id = get_uuid(TimerType::RealTime);
    let byond_tick_id = get_uuid(TimerType::ByondTick);

    assert!(matches!(real_time_id.timertype(), TimerType::RealTime));
    assert!(matches!(byond_tick_id.timertype(), TimerType::ByondTick));
}

#[test]
fn unknown_uuid_prefix_routes_to_real_time() {
    let id = Uuid::from_bytes([42; 16]);

    assert!(matches!(id.timertype(), TimerType::RealTime));
}
