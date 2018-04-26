//! Changing the panic handler
//!
//! The easiest way to change the panic handler is to use a different [panic implementation
//! crate][0].
//!
//! [0]: https://crates.io/keywords/panic-impl
//!
//! ---

#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
// extern crate panic_abort;
extern crate panic_semihosting; // reports panic messages to the host stderr using semihosting

use cortex_m::asm;
use rt::ExceptionFrame;

main!(main);

#[inline(always)]
fn main() -> ! {
    panic!("Oops")
}

exception!(DefaultHandler, deh);

#[inline(always)]
fn deh(_nr: u8) {
    asm::bkpt();
}

exception!(HardFault, hf);

#[inline(always)]
fn hf(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();

    loop {}
}

interrupts!(DefaultHandler);
