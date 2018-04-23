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
//!
//! ```
//! 
//! #![feature(alloc)]
//! #![feature(global_allocator)]
//! #![feature(used)]
//! #![no_std]
//! 
//! // This is the allocator crate; you can use a different one
//! extern crate alloc_cortex_m;
//! #[macro_use]
//! extern crate alloc;
//! extern crate cortex_m;
//! extern crate cortex_m_rt;
//! extern crate cortex_m_semihosting;
//! extern crate panic_abort; // panicking behavior
//! 
//! use core::fmt::Write;
//! 
//! use alloc_cortex_m::CortexMHeap;
//! use cortex_m::asm;
//! use cortex_m_semihosting::hio;
//! 
//! #[global_allocator]
//! static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
//! 
//! extern "C" {
//!     static mut _sheap: u32;
//! }
//! 
//! const HEAP_SIZE: usize = 1024; // in bytes
//! 
//! fn main() {
//!     // Initialize the allocator
//!     let start = unsafe { &mut _sheap as *mut u32 as usize };
//!     unsafe { ALLOCATOR.init(start, HEAP_SIZE) }
//! 
//!     // Growable array allocated on the heap
//!     let xs = vec![0, 1, 2];
//! 
//!     let mut stdout = hio::hstdout().unwrap();
//!     writeln!(stdout, "{:?}", xs).unwrap();
//! }
//! 
//! // As we are not using interrupts, we just register a dummy catch all handler
//! #[link_section = ".vector_table.interrupts"]
//! #[used]
//! static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];
//! 
//! extern "C" fn default_handler() {
//!     asm::bkpt();
//! }
//! ```
// Auto-generated. Do not modify.
