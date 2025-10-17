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

## Example Usage

### Simple Example

```dm
// Server restart system with countdown
/proc/restart_server(delay)
    // Restart the server in 5 minutes
    rt_add_timer(30000, "global", "perform_server_restart")

// The callback that performs the restart
/proc/perform_server_restart()
    world.Reboot()
```

### Advanced Example

```dm
/mob/var/burning_timer_id = null
// Set up a recurring status effect that can be cancelled
/mob/proc/apply_burning_effect(duration, damage_per_tick)
    src << "Ow! You're on fire!"

    burning_timer_id = rt_add_recurring_timer(0, 10, src, "process_burn_damage", damage_per_tick)

    // Stop the burning after the duration
    rt_add_timer(duration, src, "stop_burning")

/mob/proc/process_burn_damage(damage_amount)
    health -= damage_amount
    src << "The flames burn you for [damage_amount] damage!"

    if(health <= 0 || is_wet)
        src << "The flames are extinguished!"
        burning_timer_id = null
        return RT_TIMER_CANCEL

/mob/proc/stop_burning()
    if(burning_timer_id)
        rt_cancel_timer(burning_timer_id)
        burning_timer_id = null
        src << "The flames die down."
```

## Troubleshooting

You must build a 32-bit version of the library for it to be compatible with
BYOND. Attempting to build a 64-bit version will fail with an explanatory error.

## Windows

Debug information is automatically split & stripped into a separate `.pdb` companion
file on Windows targets.

## Linux

Debug information is automatically split & stripped into a separate `.dbg` companion
file on Linux targets. You can use the `.dbg` file for symbols (for example with `gdb`).

If your toolchain uses non-default linker names, set the environment variables
`RUSTICK_I686_REAL_LINKER` or `RUSTICK_X86_64_REAL_LINKER` to point at the desired
linker executable before invoking `cargo build`.

The split step relies on  `objcopy` & `strip`. If you use alternate toolchains,
set `RUSTICK_OBJCOPY` and `RUSTICK_STRIP` to the appropriate binaries.

[goonstation]: https://github.com/goonstation/goonstation
[Rust]: https://rust-lang.org
[Cargo]: https://doc.rust-lang.org/cargo/
[rustup]: https://rustup.rs/
[releases page]: https://github.com/goonstation/rustick/releases
[msvc]: https://visualstudio.microsoft.com/visual-cpp-build-tools/
