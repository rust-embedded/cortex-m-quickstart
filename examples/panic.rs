//! Changing the panic handler
//!
//! The easiest way to change the panic handler is to use a different [panic handler crate][0].
//!
//! [0]: https://crates.io/keywords/panic-impl
//!
//! ---

#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_rt as rt;

// Pick one of these two panic handlers:

// Reports panic messages to the host stderr using semihosting
extern crate panic_semihosting;

// Logs panic messages using the ITM (Instrumentation Trace Macrocell)
// NOTE to use this you need to uncomment the `panic-itm` dependency in Cargo.toml
// extern crate panic_itm;

use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    panic!("Oops")
}

// define the hard fault handler
exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

// define the default exception handler
exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
