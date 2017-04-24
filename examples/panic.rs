//! Redirecting `panic!` messages
//!
//! The `cortex-m-rt` crate provides two options to redirect `panic!` messages
//! through these two Cargo features:
//!
//! - `panic-over-semihosting`. `panic!` messages will be printed to the OpenOCD
//!    console using semihosting. This is slow.
//!
//! - `panic-over-itm`. `panic!` messages will be send through the ITM port 0.
//!   This is much faster but requires ITM support on the device.
//!
//! If neither of these options is specified then the `panic!` message will be
//! lost. Note that all `panic!`s will trigger a debugger breakpoint.

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;

use cortex_m::asm;

fn main() {
    panic!("Oops");
}

// As we are not using interrupts, we just register a dummy catch all handler
#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
