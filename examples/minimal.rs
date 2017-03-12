//! Minimal application

#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate {{name}};

use cortex_m::exception;
use {{name}}::interrupt;

fn main() {}

#[no_mangle]
pub static _INTERRUPTS: interrupt::Handlers =
    interrupt::Handlers { ..interrupt::DEFAULT_HANDLERS };

#[no_mangle]
pub static _EXCEPTIONS: exception::Handlers =
    exception::Handlers { ..exception::DEFAULT_HANDLERS };
