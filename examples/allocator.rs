//! How to use the heap and a dynamic memory allocator
//!
//! This example depends on the alloc-cortex-m crate so you'll have to add it to your Cargo.toml:
//!
//! ``` text
//! # or edit the Cargo.toml file manually
//! $ cargo add alloc-cortex-m
//! ```
//!
//! ---

#![feature(alloc)]
#![feature(global_allocator)]
#![no_main]
#![no_std]

// This is the allocator crate; you can use a different one
extern crate alloc_cortex_m;
#[macro_use]
extern crate alloc;
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_abort;

use core::fmt::Write;

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm;
use rt::ExceptionFrame;
use sh::hio;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

main!(main);

fn main() -> ! {
    // Initialize the allocator
    unsafe { ALLOCATOR.init(rt::heap_start() as usize, HEAP_SIZE) }

    // Growable array allocated on the heap
    let xs = vec![0, 1, 2];

    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "{:?}", xs).unwrap();

    loop {}
}

exception!(DefaultHandler, dh);

#[inline(always)]
fn dh(_nr: u8) {
    asm::bkpt();
}

exception!(HardFault, hf);

#[inline(always)]
fn hf(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();

    loop {}
}

interrupts!(DefaultHandler);
