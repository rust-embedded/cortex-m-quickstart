//! How to use the heap and a dynamic memory allocator
//!
//! This example depends on the embedded-alloc crate so you'll have to add it to your Cargo.toml:
//!
//! ``` text
//! # or edit the Cargo.toml file manually
//! $ cargo add embedded-alloc
//! ```
//!
//! ---

#![no_main]
#![no_std]

extern crate alloc;
use panic_halt as _;

use self::alloc::vec;
use core::ptr::addr_of_mut;

// Linked-List First Fit Heap allocator (feature = "llff")
use embedded_alloc::LlffHeap as Heap;
// Two-Level Segregated Fit Heap allocator (feature = "tlsf")
// use embedded_alloc::TlsfHeap as Heap;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln, debug};

// this is the allocator the application will use
#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
    }

    // Growable array allocated on the heap
    let xs = vec![0, 1, 2];

    hprintln!("{:?}", xs);

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
