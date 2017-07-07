//! Using a device crate
//!
//! Crates generated using [`svd2rust`] are referred to as device crates. These
//! crates provides an API to access the peripherals of a device. When you
//! depend on one of these crates and the "rt" feature is enabled you don't need
//! link to the cortex-m-rt crate.
//!
//! [`svd2rust`]: https://crates.io/crates/svd2rust
//!
//! Device crates also provide an `interrupt!` macro to register interrupt
//! handlers.
//!
//! This example depends on the [`stm32f103xx`] crate so you'll have to add it
//! to your Cargo.toml.
//!
//! [`stm32f103xx`]: https://crates.io/crates/stm32f103xx
//!
//! ```
//! $ edit Cargo.toml && cat $_
//! [dependencies.stm32f103xx]
//! features = ["rt"]
//! version = "0.7.0"
//! ```
//!
//! ---

#![no_std]

extern crate cortex_m;
#[macro_use(exception, interrupt)]
extern crate stm32f103xx;

use cortex_m::interrupt;

fn main() {
    interrupt::free(|cs| {
        let _gpioa = stm32f103xx::GPIOA.borrow(cs);
        // do something with GPIOA
    });
}

exception!(SYS_TICK, tick, locals: {
    ticks: u32 = 0;
});

fn tick(l: &mut SYS_TICK::Locals) {
    l.ticks += 1;
    // ..
}

interrupt!(TIM2, tock, locals: {
    tocks: u32 = 0;
});

fn tock(l: &mut TIM2::Locals) {
    l.tocks += 1;
    // ..
}
