//! How to use the heap and a dynamic memory allocator
//!
//! To compile this example you'll need to build the collections crate as part
//! of the Xargo sysroot. To do that change the Xargo.toml file to look like
//! this:
//!
//! ``` text
//! [dependencies.core]
//! stage = 0
//!
//! [dependencies.collections] # NEW
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
//! #[allow(deprecated)]
//! #![feature(collections)]
//! #![feature(used)]
//! #![no_std]
//! 
//! // This is the allocator crate; you can use a different one
//! extern crate alloc_cortex_m;
//! #[macro_use]
//! extern crate collections;
//! extern crate cortex_m;
//! extern crate cortex_m_rt;
//! extern crate cortex_m_semihosting;
//! 
//! use core::fmt::Write;
//! 
//! use cortex_m::asm;
//! use cortex_m_semihosting::hio;
//! 
//! fn main() {
//!     // Initialize the allocator
//!     unsafe {
//!         extern "C" {
//!             // Start of the heap
//!             static mut _sheap: usize;
//!         }
//! 
//!         // Size of the heap in words (1 word = 4 bytes)
//!         // NOTE The bigger the heap the greater the chance to run into a stack
//!         // overflow (collision between the stack and the heap)
//!         const SIZE: isize = 256;
//! 
//!         // End of the heap
//!         let _eheap = (&mut _sheap as *mut _).offset(SIZE);
//! 
//!         alloc_cortex_m::init(&mut _sheap, _eheap);
//!     }
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
