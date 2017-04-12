//! Minimal application

#![feature(used)]
#![no_std]

extern crate cortex_m_rt;
extern crate {{name}};

use {{name}}::interrupt;

fn main() {}

#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: interrupt::Handlers =
    interrupt::Handlers { ..interrupt::DEFAULT_HANDLERS };
