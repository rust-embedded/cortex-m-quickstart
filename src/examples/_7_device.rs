//! Using a device crate
//!
//! Crates generated using [`svd2rust`] are referred to as device crates. These crates provide an
//! API to access the peripherals of a device.
//!
//! [`svd2rust`]: https://crates.io/crates/svd2rust
//!
//! Device crates also provide an `interrupt!` macro (behind the "rt" feature) to register interrupt
//! handlers.
//!
//! This example depends on the [`stm32f103xx`] crate so you'll have to add it to your Cargo.toml.
//!
//! [`stm32f103xx`]: https://crates.io/crates/stm32f103xx
//!
//! ```
//! $ edit Cargo.toml && tail $_
//! [dependencies.stm32f103xx]
//! features = ["rt"]
//! version = "0.10.0"
//! ```
//!
//! ---
//!
//! ```
//!
//! #![no_main]
//! #![no_std]
//!
//! extern crate cortex_m;
//! #[macro_use]
//! extern crate cortex_m_rt as rt;
//! extern crate cortex_m_semihosting as sh;
//! #[macro_use]
//! extern crate stm32f103xx;
//! extern crate panic_semihosting;
//!
//! use core::fmt::Write;
//!
//! use cortex_m::peripheral::syst::SystClkSource;
//! use rt::ExceptionFrame;
//! use sh::hio::{self, HStdout};
//! use stm32f103xx::Interrupt;
//!
//! entry!(main);
//!
//! fn main() -> ! {
//!     let p = cortex_m::Peripherals::take().unwrap();
//!
//!     let mut syst = p.SYST;
//!     let mut nvic = p.NVIC;
//!
//!     nvic.enable(Interrupt::EXTI0);
//!
//!     // configure the system timer to wrap around every second
//!     syst.set_clock_source(SystClkSource::Core);
//!     syst.set_reload(8_000_000); // 1s
//!     syst.enable_counter();
//!
//!     loop {
//!         // busy wait until the timer wraps around
//!         while !syst.has_wrapped() {}
//!
//!         // trigger the `EXTI0` interrupt
//!         nvic.set_pending(Interrupt::EXTI0);
//!     }
//! }
//!
//! // try commenting out this line: you'll end in `default_handler` instead of in `exti0`
//! interrupt!(EXTI0, exti0, state: Option<HStdout> = None);
//!
//! fn exti0(state: &mut Option<HStdout>) {
//!     if state.is_none() {
//!         *state = Some(hio::hstdout().unwrap());
//!     }
//!
//!     if let Some(hstdout) = state.as_mut() {
//!         hstdout.write_str(".").unwrap();
//!     }
//! }
//!
//! exception!(HardFault, hard_fault);
//!
//! fn hard_fault(ef: &ExceptionFrame) -> ! {
//!     panic!("HardFault at {:#?}", ef);
//! }
//!
//! exception!(*, default_handler);
//!
//! fn default_handler(irqn: i16) {
//!     panic!("Unhandled exception (IRQn = {})", irqn);
//! }
//! ```
// Auto-generated. Do not modify.
