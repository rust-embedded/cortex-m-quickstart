//! Changing the panicking behavior
//!
//! The easiest way to change the panicking behavior is to use a different [panic handler crate][0].
//!
//! [0]: https://crates.io/keywords/panic-impl

#![no_main]
#![no_std]

// Pick one of these panic handlers:

// `panic!` halts execution; the panic message is ignored
extern crate panic_halt;

// Reports panic messages to the host stderr using semihosting
// NOTE to use this you need to uncomment the `panic-semihosting` dependency in Cargo.toml
// extern crate panic_semihosting;

// Logs panic messages using the ITM (Instrumentation Trace Macrocell)
// NOTE to use this you need to uncomment the `panic-itm` dependency in Cargo.toml
// extern crate panic_itm;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    panic!("Oops")
}
