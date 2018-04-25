//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---

#![feature(asm)]
#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_abort; // panicking behavior

use core::fmt::Write;

use sh::hio;

main!(main);

fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();

    loop {}
}

exception!(DefaultHandler, dh);

fn dh(_nr: u8) -> ! {
    loop {}
}

// As we are not using interrupts, we just bind them all to the `DefaultHandler` exception handler
interrupts!(DefaultHandler);
