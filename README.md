# rustick

It's time


dm junk: 
```
#define add_timer(delay, proc_owner, proc_name, proc_args...) call_ext("project1","byond:schedule_once")(delay, proc_owner, proc_name, list(proc_args))
#define add_recurring_timer(delay, period, proc_owner, proc_name, proc_args...) call_ext("project1","byond:schedule_periodic")(delay, period, proc_owner, proc_name, list(proc_args))

/proc/cancel_timer(var/id)
	call_ext("project1","byond:cancel_timer")(id)

/proc/timer_error(var/a)
	stack_trace("Timer error: [a]")

//for dev memes:

/proc/start_timer_proc_test()
	boutput(world, "[world.time] Scheduling a bunch of timers")
	for (var/i in 1 to 10)
		var/d = 1000*i
		call_ext("project1","byond:schedule_once")(d, "global", "timer_test", list("I was scheduled at [world.time] for [d]"))
		//call_ext("project1","byond:schedule_once")(d, "notaref", "timer_test", list("I was scheduled at [world.time] for [d]"))
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


```
