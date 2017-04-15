//! Device agnostic "Hello, world!"
//!
//! Prints "Hello, world!" on the OpenOCD console using semihosting

#![feature(used)]
#![no_std]

extern crate cortex_m_rt;

fn main() {}

#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: [u32; 240] = [0; 240];
