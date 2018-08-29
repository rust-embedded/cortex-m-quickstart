//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---

#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;

use core::fmt::Write;

use sh::hio;

entry!(main);

fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();

    loop {}
}
