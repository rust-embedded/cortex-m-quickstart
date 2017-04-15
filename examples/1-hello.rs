//! Device specific version of "Hello, world!"
//!
//! Prints "Hello, world!" on the OpenOCD console using semihosting

#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate {{name}};

use {{name}}::interrupt;

fn main() {
    hprintln!("Hello, world!");
}

// This is the device specific bit: properly populated interrupt handlers
// Tough we are not using any of them in this example
#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: interrupt::Handlers =
    interrupt::Handlers { ..interrupt::DEFAULT_HANDLERS };
