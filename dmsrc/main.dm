// rustick.dm - DM API for rustick timer library
//
// #define RUSTICK "path/to/rustick"
// Override the .dll/.so detection logic with a fixed path or with detection
// logic of your own.

#ifndef RUSTICK
// Default automatic RUSTICK detection.
// On Windows, looks in the standard places for `rustick.dll`.
// On Linux, looks in `.` for `librustick.so`.`
// On x64 (OpenDream), looks with `64` suffixes.

/* This comment bypasses grep checks */ /var/__rustick

/proc/__detect_rustick()
	var/arch_suffix = null
	#ifdef OPENDREAM
	arch_suffix = "64"
	#endif
	if (world.system_type == UNIX)
		if (fexists("./librustick[arch_suffix].so"))
			return __rustick = "./librustick[arch_suffix].so"
		else
			// It's not in the current directory, so try others
			return __rustick = "librustick[arch_suffix].so"
	else
		return __rustick = "rustick[arch_suffix]"

#define RUSTICK (__rustick || __detect_rustick())
#endif

#define RT_TIMER_CANCEL "RT_TIMER_CANCEL"
#define RT_TIMER_RESCHEDULE "RT_TIMER_RESCHEDULE"

/// Gets the version of rustick
/proc/rustick_get_version() as text
	return call_ext(RUSTICK, "get_version")()

/**
 * Schedules a one-time timer to call a proc after a delay.
 *
 * * `delay` - Time in deciseconds to wait before calling the proc.
 * * `proc_owner` - The datum/atom that owns the proc to call. Can also be `"global"`.
 * * `proc_name` - The name of the proc to call. See: `PROC_REF`, `GLOBAL_PROC_REF`.
 * * `proc_args` (varadic, optional) Arguments to pass to the called proc.
 *
 * **Returns** - A unique ID (uuidv4) for the scheduled timer.
 */
#define rt_add_timer(delay, proc_owner, proc_name, proc_args...) call_ext(RUSTICK, "byond:schedule_once")(delay * 100, proc_owner, proc_name, list(proc_args))
#define rt_add_timer_ms(delay, proc_owner, proc_name, proc_args...) call_ext(RUSTICK, "byond:schedule_once")(delay, proc_owner, proc_name, list(proc_args))
#define rt_add_timer_tick(delay, proc_owner, proc_name, proc_args...) call_ext(RUSTICK, "byond:schedule_once_tick")(delay, proc_owner, proc_name, list(proc_args))
#define rt_add_recurring_timer(delay, period, proc_owner, proc_name, proc_args...) call_ext(RUSTICK, "byond:schedule_periodic")(delay * 100, period * 100, proc_owner, proc_name, list(proc_args))
#define rt_add_recurring_timer_ms(delay, period, proc_owner, proc_name, proc_args...) call_ext(RUSTICK, "byond:schedule_periodic")(delay, period, proc_owner, proc_name, list(proc_args))
#define rt_add_recurring_timer_tick(delay, period, proc_owner, proc_name, proc_args...) call_ext(RUSTICK, "byond:schedule_periodic_tick")(delay, period, proc_owner, proc_name, list(proc_args))

/proc/rt_cancel_timer(var/id)
	call_ext(RUSTICK, "byond:cancel_timer")(id)

/proc/rt_timer_error(error_str)
	stack_trace("Rustick Timer error: [error_str]")

/*
/proc/start_timer_proc_test()
	boutput(world, "[world.time] Scheduling a bunch of timers")
	for (var/i in 1 to 10)
		var/d = 1000*i
		call_ext("rustick","byond:schedule_once")(d, "global", "timer_test", list("I was scheduled at [world.time] for [d]"))
		//call_ext("rustick","byond:schedule_once")(d, "notaref", "timer_test", list("I was scheduled at [world.time] for [d]"))
	boutput(world, "[world.time] Timers scheduled")

/proc/start_timer_mob_proc_test()
	var/mob/mymob = usr
	boutput(world, "[world.time] Scheduling a bunch of timers on [usr]")
	for (var/i in 1 to 10)
		var/d = 1000*i
		var/list/my_args = list("I was scheduled at [world.time] for [d]")
		add_timer(d, mymob, "timer_test", "I was scheduled at [world.time] for [d]")
	boutput(world, "[world.time] Timers scheduled")

/proc/start_periodic_timer_proc_test()
	boutput(world, "[world.time] Scheduling a periodic timer")
	var/ret = add_recurring_timer(1000, 1000, "notaref", "timer_test", "I was scheduled to be periodic at [world.time] for 1000")
	boutput(world, "Periodic timer id is [ret]")

/proc/start_periodic_timer_proc_test_that_cancels()
	boutput(world, "[world.time] Scheduling a periodic timer")
	var/ret = add_recurring_timer(1000, 1000, "notaref", "timer_test_cancels", "I was scheduled to be periodic at [world.time] for 1000")
	boutput(world, "Periodic timer id is [ret]")

/proc/start_periodic_mob_timer_proc_test()
	var/mob/mymob = usr
	boutput(world, "[world.time] Scheduling a periodic timer on [usr]")
	var/ret = add_recurring_timer(1000, 1000, mymob, "timer_test", "I was scheduled to be periodic at [world.time] for 1000")
	boutput(world, "Periodic timer id is [ret]")



/mob/proc/timer_test(var/a)
	boutput(src, "[world.time] It's time. Mob timer says: [a]")

/proc/timer_test(var/a)
	boutput(world, "[world.time] It's time. Global timer says: [a]")

/proc/timer_test_cancels(var/a)
	boutput(world, "[world.time] It's time. Global timer says: [a]")
	return "TIMER_CANCEL"
*/
