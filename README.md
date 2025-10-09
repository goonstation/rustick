# rustick

rustick (pronounced rustic) is a library that implements precise timers
and periodic scheduling for BYOND games written in DM.

This library is currently used in the [goonstation] codebase, and is required for
it to run. A pre-compiled DLL version can be found in the repo root of codebases that use it,
but you can build your own from this repo (and __you should__ if you're running a server).

Builds can also be found on the [releases page] **but should only be used for Windows**,
as Linux has compatibility issues across distributions.

## Dependencies

The [Rust] compiler:

1. Use [the Rust installer](https://rustup.rs/), or another Rust installation method,
   or run the following:

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    You may need to install the Visual Studio C++ Build tools (system linker) when prompted to do so.

2. Add the **32-bit** compiler target:

    ```sh
    git clone https://github.com/goonstation/rustick.git
    cd rustick
    # Linux
    rustup target add i686-unknown-linux-gnu
    # Windows
    rustup target add i686-pc-windows-msvc
    ```

## Compiling

The [Cargo] tool handles compilation, as well as automatically downloading and
compiling all Rust dependencies. The default configuration is suitable for
use with the [goonstation] codebase. To compile in release mode (recommended for
speed):

Linux:
```sh
cargo build --release --target i686-unknown-linux-gnu
# output: target/i686-unknown-linux-gnu/release/librustick.so
```

Windows:

If you are using Visual Studio Code, you may use the `CONTROL + SHIFT + B` hotkey and run the `rust: cargo build (win32)` task.

Alternatively:
```sh
cargo build --release --target i686-pc-windows-msvc
# output: target/i686-pc-windows-msvc/release/rustick.dll
```

If you aren't sharing the binary with other people, consider compiling [targeting your native cpu](https://rust-lang.github.io/packed_simd/perf-guide/target-feature/rustflags.html#target-cpu) for potential performance improvements. You can do this by setting the `RUSTFLAGS` environment variable to `-C target-cpu=native`. For example, in Powershell you would use `$Env:RUSTFLAGS="-C target-cpu=native"`.

## Features

To get additional features, pass a list to `--features`, for example `--features allow_non_32bit`.

* `allow_non_32bit`: Disables the forced compile errors on non-32bit targets. Only use this if you know exactly what you are doing.

## Installing

The rustick binary (`rustick.dll` or `librustick.so`) should be placed in the root
of your repository next to your `.dmb`. There are alternative installation
locations, but this one is best supported.

Compiling will also create the file `target/rustick.dm` which contains the DM API.
To use rustick, copy-paste this file into your project.

## Troubleshooting

You must build a 32-bit version of the library for it to be compatible with
BYOND. Attempting to build a 64-bit version will fail with an explanatory error.

dm junk:
```dm
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

[goonstation]: https://github.com/goonstation/goonstation
[Rust]: https://rust-lang.org
[Cargo]: https://doc.rust-lang.org/cargo/
[rustup]: https://rustup.rs/
[releases page]: https://github.com/goonstation/rustick/releases
[msvc]: https://visualstudio.microsoft.com/visual-cpp-build-tools/
