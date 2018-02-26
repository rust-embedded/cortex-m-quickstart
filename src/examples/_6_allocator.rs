//! How to use the heap and a dynamic memory allocator
//!
//! To compile this example you'll need to build the alloc crate as part
//! of the Xargo sysroot. To do that change the Xargo.toml file to look like
//! this:
//!
//! ``` text
//! [dependencies.core]
//! stage = 0
//!
//! [dependencies.alloc] # NEW
//! stage = 0
//!
//! [dependencies.compiler_builtins]
//! stage = 1
//! ```
//!
//! This example depends on the alloc-cortex-m crate so you'll have to add it
//! to your Cargo.toml:
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
//! #![feature(used)]
//! #![feature(global_allocator)]
//! #![no_std]
//! 
//! // This is the allocator crate; you can use a different one
//! extern crate alloc_cortex_m;
//! #[macro_use]
//! extern crate alloc;
//! extern crate cortex_m;
//! extern crate cortex_m_rt;
//! extern crate cortex_m_semihosting;
//! 
//! use core::fmt::Write;
//! 
//! use cortex_m::asm;
//! use cortex_m_semihosting::hio;
//! use alloc_cortex_m::CortexMHeap;
//! 
//! #[global_allocator]
//! static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
//! 
//! extern "C" {
//!     static mut _sheap: u32;
//!     static mut _eheap: u32;
//! }
//! 
//! fn main() {
//!     // Initialize the allocator
//!     let start = unsafe { &mut _sheap as *mut u32 as usize };
//!     let end = unsafe { &mut _eheap as *mut u32 as usize };
//!     unsafe { ALLOCATOR.init(start, end - start) }
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
