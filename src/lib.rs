//! A template for building applications for ARM Cortex-M microcontrollers
//!
//! # Dependencies
//!
//! - Nightly Rust toolchain: `rustup default nightly`
//! - ARM linker: `sudo apt-get install binutils-arm-none-eabi` (on Ubuntu)
//! - Cargo `clone` subcommand: `cargo install cargo-clone`
//! - GDB: `sudo apt-get install gdb-arm-none-eabi` (on Ubuntu)
//! - OpenOCD: `sudo apt-get install OpenOCD` (on Ubuntu)
//! - Xargo: `cargo install xargo`
//! - [Optional] Cargo `add` subcommand: `cargo install cargo-edit`
//!
//! # Usage
//!
//! 1) Clone this crate
//!
//! ``` text
//! $ cargo clone cortex-m-quickstart && cd $_
//! ```
//!
//! 2) Change the crate name, author and version
//!
//! ``` text
//! $ edit Cargo.toml && head $_
//! [package]
//! authors = ["Jorge Aparicio <jorge@japaric.io>"]
//! name = "demo"
//! version = "0.1.0"
//! ```
//!
//! 3) Specify the memory layout of the target device
//!
//! **NOTE** board support crates sometimes provide this file for you (check the crate
//! documentation). If you are using one that does then remove *both* the `memory.x` and `build.rs`
//! files.
//!
//! ``` text
//! $ edit memory.x && cat $_
//! MEMORY
//! {
//!   /* NOTE K = KiBi = 1024 bytes */
//!   FLASH : ORIGIN = 0x08000000, LENGTH = 256K
//!   RAM : ORIGIN = 0x20000000, LENGTH = 40K
//! }
//! ```
//!
//! 4) Optionally, set a default build target
//!
//! ``` text
//! $ cat >>.cargo/config <<'EOF'
//! [build]
//! target = "thumbv7em-none-eabihf"
//! EOF
//! ```
//!
//! 5) Depend on a device, HAL implementation or a board support crate.
//!
//! ``` text
//! $ # add a device crate, OR
//! $ cargo add stm32f30x
//!
//! $ # add a HAL implementation crate, OR
//! $ cargo add stm32f103xx-hal
//!
//! $ # add a board support crate
//! $ cargo add f3
//! ```
//!
//! 6) Write the application or start from one of the examples
//!
//! ``` text
//! $ rm -r src/* && cp examples/hello.rs src/main.rs
//! ```
//!
//! 7) Build the application
//!
//! ``` text
//! $ # NOTE this command requires `arm-none-eabi-ld` to be in $PATH
//! $ xargo build --release
//!
//! $ # sanity check
//! $ arm-none-eabi-readelf -A target/thumbv7em-none-eabihf/release/demo
//! Attribute Section: aeabi
//! File Attributes
//!   Tag_conformance: "2.09"
//!   Tag_CPU_arch: v7E-M
//!   Tag_CPU_arch_profile: Microcontroller
//!   Tag_THUMB_ISA_use: Thumb-2
//!   Tag_FP_arch: VFPv4-D16
//!   Tag_ABI_PCS_GOT_use: direct
//!   Tag_ABI_FP_denormal: Needed
//!   Tag_ABI_FP_exceptions: Needed
//!   Tag_ABI_FP_number_model: IEEE 754
//!   Tag_ABI_align_needed: 8-byte
//!   Tag_ABI_align_preserved: 8-byte, except leaf SP
//!   Tag_ABI_HardFP_use: SP only
//!   Tag_ABI_VFP_args: VFP registers
//!   Tag_ABI_optimization_goals: Aggressive Speed
//!   Tag_CPU_unaligned_access: v6
//!   Tag_FP_HP_extension: Allowed
//!   Tag_ABI_FP_16bit_format: IEEE 754
//! ```
//!
//! 8) Flash the program
//!
//! ``` text
//! $ # Launch OpenOCD on a terminal
//! $ openocd -f (..)
//! ```
//!
//! ``` text
//! $ # Start a debug session in another terminal
//! $ arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/demo
//! ```
//!
//! **NOTE** As of nightly-2017-05-14 or so and cortex-m-quickstart v0.1.6 you can simply run `xargo
//! run` or `xargo run --example $example` to build the program, *and* immediately start a debug
//! session. IOW, it lets you omit the `arm-none-eabi-gdb` command.
//!
//! ``` text
//! $ cargo run --example hello
//! > # drops you into a GDB session
//! ```
//!
//! # Examples
//!
//! Check the [examples module](./examples/index.html)
//!
//! # Troubleshooting
//!
//! This section contains fixes for common errors encountered when the
//! `cortex-m-quickstart` template is misused.
//!
//! ## Forgot to launch an OpenOCD instance
//!
//! Error message:
//!
//! ``` text
//! $ arm-none-eabi-gdb target/..
//! Reading symbols from hello...done.
//! .gdbinit:1: Error in sourced command file:
//! :3333: Connection timed out.
//! ```
//!
//! Solution: Launch OpenOCD on other terminal. See [Usage] section.
//!
//! [Usage]: ./index.html#usage
//!
//! ## Didn't modify the `memory.x` linker script
//!
//! Error message:
//!
//! ``` text
//! $ xargo build
//! Compiling demo v0.1.0 (file:///home/japaric/tmp/demo)
//! error: linking with `arm-none-eabi-ld` failed: exit code: 1
//! |
//! = note: "arm-none-eabi-ld" "-L" (..)
//! = note: arm-none-eabi-ld: address 0xbaaab838 of hello section `.text' is ..
//! arm-none-eabi-ld: address 0xbaaab838 of hello section `.text' is ..
//! arm-none-eabi-ld:
//! Invalid '.rodata.exceptions' section.
//! Make sure to place a static with type `cortex_m::exception::Handlers`
//! in that section (cf. #[link_section]) ONLY ONCE.
//! ```
//!
//! Solution: Specify your device memory layout in the `memory.x` linker script.
//! See [Usage] section.
//!
//! ## Forgot to set a default build target
//!
//! Error message:
//!
//! ``` text
//! $ xargo build
//! (..)
//! Compiling cortex-m-semihosting v0.1.3
//! error[E0463]: can't find crate for `std`
//!
//! error: aborting due to previous error
//! ```
//!
//! Solution: Set a default build target in the `.cargo/config` file
//! (see [Usage] section), or call Xargo with `--target` flag:
//! `xargo build --target thumbv7em-none-eabi`.
//!
//! ## Overwrote the original `.cargo/config` file
//!
//! Error message:
//!
//! ``` text
//! error: linking with `arm-none-eabi-gcc` failed: exit code: 1
//!   |
//!   = note: (..)
//! (..)
//! (..)/crt0.o: In function `_start':
//! (.text+0x90): undefined reference to `memset'
//! (..)/crt0.o: In function `_start':
//! (.text+0xd0): undefined reference to `atexit'
//! (..)/crt0.o: In function `_start':
//! (.text+0xd4): undefined reference to `__libc_init_array'
//! (..)/crt0.o: In function `_start':
//! (.text+0xe4): undefined reference to `exit'
//! (..)/crt0.o: In function `_start':
//! (.text+0x100): undefined reference to `__libc_fini_array'
//! collect2: error: ld returned 1 exit status
//! ```
//!
//! Solution: You probably overwrote the original `.cargo/config` instead of
//! appending the default build target (e.g. `cat >` instead of `cat >>`). The
//! less error prone way to fix this is to remove the `.cargo` directory, clone
//! a new copy of the template and then copy the `.cargo` directory from that
//! fresh template into your current project. Don't forget to *append* the
//! default build target to `.cargo/config`.
//!
//! ## Called OpenOCD with wrong arguments
//!
//! Error message:
//!
//! ``` text
//! $ openocd -f ..
//! (..)
//! Error: open failed
//! in procedure 'init'
//! in procedure 'ocd_bouncer'
//! ```
//!
//! Solution: Correct the OpenOCD arguments. Check the
//! `/usr/share/openocd/scripts` directory (exact location varies per
//! distribution / OS) for a list of scripts that can be used.
//!
//! ## Used Cargo instead of Xargo
//!
//! Error message:
//!
//! ``` text
//! $ cargo build
//! Compiling cortex-m-rt v0.2.0
//! error[E0463]: can't find crate for `core`
//! |
//! = note: the `thumbv7em-none-eabihf` target may not be installed
//!
//! error: aborting due to previous error
//! ```
//!
//! Solution: Use `xargo build`.
//!
//! ## Used the stable toolchain
//!
//! Error message:
//!
//! ``` text
//! $ xargo build
//! error: failed to run `rustc` to learn about target-specific information
//!
//! To learn more, run the command again with --verbose.
//! ```
//!
//! Solution: Switch to the nightly toolchain with `rustup default nightly`.
//!
//! ## Used `gdb` instead of `arm-none-eabi-gdb`
//!
//! Error message:
//!
//! ``` text
//! $ gdb target/..
//! Reading symbols from hello...done.
//! warning: Architecture rejected target-supplied description
//! warning: Cannot convert floating-point register value to ..
//! value has been optimized out
//! Cannot write the dashboard
//! Traceback (most recent call last):
//! File "<string>", line 353, in render
//! File "<string>", line 846, in lines
//! gdb.error: Frame is invalid.
//! 0x00000000 in ?? ()
//! semihosting is enabled
//! Loading section .text, size 0xd88 lma 0x8000000
//! Start address 0x8000000, load size 3464
//! .gdbinit:6: Error in sourced command file:
//! Remote connection closed
//! ```
//!
//! Solution: Use `arm-none-eabi-gdb target/..`
//!
//! # Used a named piped for `itm.fifo`
//!
//! Error message:
//!
//! ``` text
//! $ xargo run [--example ..]
//!
//! Reading symbols from target/thumbv7em-none-eabihf/debug/cortex-m-quickstart...done.
//! cortex_m_rt::reset_handler ()
//!     at $REGISTRY/cortex-m-rt-0.3.12/src/lib.rs:330
//! 330     unsafe extern "C" fn reset_handler() -> ! {
//! semihosting is enabled
//! Ignoring packet error, continuing...
//! Ignoring packet error, continuing...
//! ```
//!
//! Note that when you reach this point OpenOCD will become unresponsive and you'll have to kill it
//! and start a new OpenOCD process before you can invoke `xargo run` / start GDB.
//!
//! Cause: You uncommented the `monitor tpiu ..` line in `.gdbinit` and are using a named pipe to
//! receive the ITM data (i.e. you ran `mkfifo itm.fifo`). This error occurs when `itmdump -f
//! itm.fifo` (or equivalent, e.g. `cat itm.fifo`) is not running.
//!
//! Solution: Run `itmdump -f itm.fifo` (or equivalently `cat itm.fifo`) *before* invoking `xargo
//! run` / starting GDB. Note that sometimes `itmdump` will exit when the GDB session ends. In that
//! case you'll have to run `itmdump` before you start the next GDB session.
//!
//! Alternative solution: Use a plain text file instead of a named pipe. In this scenario you omit
//! the `mkfifo itm.dump` command. You can use `itmdump`'s *follow* mode (-F) to get named pipe like
//! output.

#![no_std]

pub mod examples;
