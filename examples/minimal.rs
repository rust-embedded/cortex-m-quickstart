#![no_main] // <- IMPORTANT!
#![no_std]

#[macro_use]
extern crate cortex_m_rt;
extern crate panic_abort;

use core::ptr;

// user entry point
main!(main);

fn main() -> ! {
    loop {
        unsafe {
            // NOTE side affect to avoid UB
            ptr::read_volatile(0x2000_0000 as *const u32);
        }
    }
}

// define the default exception handler
exception!(DefaultHandler, deh);

fn deh(_nr: u8) -> ! {
    loop {
        unsafe {
            // NOTE side affect to avoid UB
            ptr::read_volatile(0x2000_0004 as *const u32);
        }
    }
}

// bind all interrupts to the default exception handler
interrupts!(DefaultHandler);
