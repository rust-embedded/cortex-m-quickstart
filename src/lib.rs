//! A template for building applications for ARM Cortex-M microcontrollers
//!
//! # Dependencies
//!
//! - Nightly Rust toolchain: `rustup default nightly`
//! - ARM linker: `sudo apt-get install binutils-arm-none-eabi`
//! - Cargo `clone` subcommand: `cargo install cargo-clone`
//! - GDB: `sudo apt-get install gdb-arm-none-eabi`
//! - OpenOCD: `sudo apt-get install OpenOCD`
//! - Xargo: `cargo install xargo`
//! - [Optional] Cargo `add` subcommand: `cargo install cargo-edit`
//!
//! # Usage
//!
//! - Clone this crate
//!
//! ``` text
//! $ cargo clone cortex-m-quickstart && cd $_
//! ```
//!
//! - Change the crate name, author and version
//!
//! ``` text
//! $ edit Cargo.toml && head $_
//! [package]
//! authors = ["Jorge Aparicio <jorge@japaric.io>"]
//! name = "demo"
//! version = "0.1.0"
//! ```
//!
//! - Specify the memory layout of the target device
//!
//! (Note that some board support crates may provide this file for you (check
//! the crate documentation). If you are using one that does that then remove
//! *both* the `memory.x` and `build.rs` files.)
//!
//! ``` text
//! $ edit memory.x && cat $_
//! MEMORY
//! {
//!   /* NOTE K = KiBi = 1024 bytes */
//!   FLASH : ORIGIN = 0x08000000, LENGTH = 256K
//!   RAM : ORIGIN = 0x20000000, LENGTH = 40K
//! }
//!
//! /* This is where the call stack will be allocated. */
//! /* The stack is of the full descending type. */
//! /* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
//! _stack_start = ORIGIN(RAM) + LENGTH(RAM);
//! ```
//!
//! - Optionally, set a default build target
//!
//! ``` text
//! $ cat >>.cargo/config <<'EOF'
//! [build]
//! target = "thumbv7em-none-eabihf"
//! EOF
//! ```
//!
//! - Very likely, depend on a device or a BSP (Board Support Package) crate.
//!
//! ``` text
//! # add a device crate, or
//! $ cargo add stm32f30x
//!
//! # add a board support crate
//! $ cargo add f3
//! ```
//!
//! - Write the application or start from one of the examples
//!
//! ``` text
//! $ rm -r src/* && cp examples/hello.rs src/main.rs
//! ```
//!
//! - Disable incremental compilation. It doesn't work for embedded development.
//!   You'll hit nonsensical linker errors if you use it.
//!
//! ``` text
//! $ unset CARGO_INCREMENTAL
//! ```
//!
//! - Build the application
//!
//! ``` text
//! # NOTE this command requires `arm-none-eabi-ld` to be in $PATH
//! $ xargo build --release
//!
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
//! - Flash the program
//!
//! ```
//! # Launch OpenOCD on a terminal
//! $ openocd -f (..)
//! ```
//!
//! ```
//! # Start debug session
//! $ arm-none-eabi-gdb target/..
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
//! ## Used `CARGO_INCREMENTAL=1`
//!
//! Error message:
//!
//! ```
//! $ xargo build
//! error: linking with `arm-none-eabi-ld` failed: exit code: 1
//!     |
//! = note: "arm-none-eabi-ld" (..)
//!     = note: arm-none-eabi-ld:
//! You must specify the exception handlers.
//!     Create a non `pub` static variable with type
//!     `cortex_m::exception::Handlers` and place it in the
//!     '.rodata.exceptions' section. (cf. #[link_section]). Apply the
//!     `#[used]` attribute to the variable to make it reach the linker.
//!     arm-none-eabi-ld:
//! Invalid '.rodata.exceptions' section.
//!     Make sure to place a static with type `cortex_m::exception::Handlers`
//!     in that section (cf. #[link_section]) ONLY ONCE.
//! ```
//!
//! Solution: `$ unset CARGO_INCREMENAL`. And to be on the safe side, call
//! `cargo clean` and thrash the Xargo sysroot: `$ rm -rf ~/.xargo`
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

#![no_std]

pub mod examples;
