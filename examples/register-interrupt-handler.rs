//! Register an interrupt handler
//!
//! NOTE Requires a device crate generated using `svd2rust`

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
// NOTE this is the device crate
extern crate stm32f30x;

use cortex_m::asm;
use stm32f30x::interrupt;

fn main() {}

// NOTE each interrupt handler has a different signature
extern "C" fn my_interrupt_handler(_ctxt: interrupt::Tim7) {
    asm::bkpt();
}

extern "C" fn another_interrupt_handler(_ctxt: interrupt::Exti0) {
    asm::bkpt();
}

// Here we override only two interrupt handlers, the rest of interrupt are
// handled by the same interrupt handler
#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: interrupt::Handlers = interrupt::Handlers {
    Tim7: my_interrupt_handler,
    Exti0: another_interrupt_handler,
    ..interrupt::DEFAULT_HANDLERS
};
