#![no_main] // <- IMPORTANT!
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate panic_abort;

use cortex_m::asm;
use rt::ExceptionFrame;

// user entry point
main!(main);

#[inline(always)]
fn main() -> ! {
    asm::bkpt();

    loop {}
}

// define the default exception handler
exception!(DefaultHandler, deh);

#[inline(always)]
fn deh(_nr: u8) {
    asm::bkpt();
}

// define the hard fault handler
exception!(HardFault, hf);

#[inline(always)]
fn hf(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();

    loop {}
}

// bind all interrupts to the default exception handler
interrupts!(DefaultHandler);
