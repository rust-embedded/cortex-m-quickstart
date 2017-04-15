//! Overriding an exception
//!
//! **NOTE** You have to disable the `cortex-m-rt` crate "exceptions" feature to
//! make this work.

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate {{name}};

use core::ptr;

use cortex_m::{asm, exception};
use {{name}}::interrupt;

fn main() {
    unsafe {
        // Invalid memory access
        ptr::read_volatile(0x2FFF_FFFF as *const u32);
    }
}

extern "C" fn hard_fault(_: exception::HardFault) {
    // You'll hit this breakpoint rather than the one in cortex-m-rt
    asm::bkpt()
}

// When the "exceptions" feature is disabled, you'll have to provide this symbol
#[allow(dead_code)]
#[used]
#[link_section = ".rodata.exceptions"]
static EXCEPTIONS: exception::Handlers = exception::Handlers {
    // This is the exception handler override
    hard_fault: hard_fault,
    ..exception::DEFAULT_HANDLERS
};

#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: interrupt::Handlers =
    interrupt::Handlers { ..interrupt::DEFAULT_HANDLERS };
