//! Minimal Cortex-M program
//!
//! When executed this program will hit the breakpoint set in `main`.
//!
//! All Cortex-M programs need to:
//!
//! - Contain the `#![no_main]` and `#![no_std]` attributes. Embedded programs don't use the
//! standard Rust `main` interface or the Rust standard (`std`) library.
//!
//! - Define their entry point using [`entry!`] macro.
//!
//! [`entry!`]: https://docs.rs/cortex-m-rt/~0.5/cortex_m_rt/macro.entry.html
//!
//! - Define their panicking behavior, i.e. what happens when `panic!` is called. The easiest way to
//! define a panicking behavior is to link to a [panic handler crate][0]
//!
//! [0]: https://crates.io/keywords/panic-impl

#![no_main] // <- IMPORTANT!
#![no_std]

extern crate cortex_m;

#[macro_use(entry)]
extern crate cortex_m_rt as rt;

// makes `panic!` print messages to the host stderr using semihosting
extern crate panic_semihosting;

use cortex_m::asm;

// the program entry point is ...
entry!(main);

// ... this never ending function
fn main() -> ! {
    loop {
        asm::bkpt();
    }
}
