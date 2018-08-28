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
#![feature(alloc_error_handler)]
#![feature(lang_items)]
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
extern crate panic_semihosting;

use core::alloc::Layout;
use core::fmt::Write;

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm;
use rt::ExceptionFrame;
use sh::hio;

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

entry!(main);

fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    unsafe { ALLOCATOR.init(rt::heap_start() as usize, HEAP_SIZE) }

    // Growable array allocated on the heap
    let xs = vec![0, 1, 2];

    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "{:?}", xs).unwrap();

    loop {}
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
#[no_mangle]
pub fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
