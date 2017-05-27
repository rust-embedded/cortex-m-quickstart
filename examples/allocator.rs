//! How to use the heap and a dynamic memory allocator
//!
//! To compile this example you'll need to build the collections crate as part
//! of the Xargo sysroot. To do that change the Xargo.toml file to look like
//! this:
//!
//! ``` text
//! [dependencies.core]
//! [dependencies.collections] # new
//!
//! [dependencies.compiler_builtins]
//! features = ["mem"]
//! git = "https://github.com/rust-lang-nursery/compiler-builtins"
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

#![feature(collections)]
#![feature(used)]
#![no_std]

// This is the allocator crate; you can use a different one
extern crate alloc_cortex_m;
#[macro_use]
extern crate collections;
#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;

use cortex_m::asm;

fn main() {
    // Initialize the allocator
    unsafe {
        extern "C" {
            // Start of the heap
            static mut _sheap: usize;
        }

        // Size of the heap in words (1 word = 4 bytes)
        // WARNING: The bigger the heap the greater the chance to run into a
        // stack overflow (collision between the stack and the heap)
        const SIZE: isize = 256;

        // End of the heap
        let _eheap = (&mut _sheap as *mut _).offset(SIZE);

        alloc_cortex_m::init(&mut _sheap as *mut _, _eheap);
    }

    // Growable array allocated on the heap
    let xs = vec![0, 1, 2];
    hprintln!("{:?}", xs);
}

// As we are not using interrupts, we just register a dummy catch all handler
#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
