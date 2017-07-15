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
//!
//! ```
//! 
//! #![deny(warnings)]
//! #![feature(const_fn)]
//! #![no_std]
//! 
//! extern crate cortex_m;
//! extern crate cortex_m_semihosting;
//! #[macro_use(exception, interrupt)]
//! extern crate stm32f103xx;
//! 
//! use core::cell::RefCell;
//! use core::fmt::Write;
//! 
//! use cortex_m::interrupt::{self, Mutex};
//! use cortex_m::peripheral::SystClkSource;
//! use cortex_m_semihosting::hio::{self, HStdout};
//! use stm32f103xx::Interrupt;
//! 
//! static HSTDOUT: Mutex<RefCell<Option<HStdout>>> =
//!     Mutex::new(RefCell::new(None));
//! 
//! fn main() {
//!     interrupt::free(|cs| {
//!         let hstdout = HSTDOUT.borrow(cs);
//!         if let Ok(fd) = hio::hstdout() {
//!             *hstdout.borrow_mut() = Some(fd);
//!         }
//! 
//!         let nvic = stm32f103xx::NVIC.borrow(cs);
//!         nvic.enable(Interrupt::TIM2);
//! 
//!         let syst = stm32f103xx::SYST.borrow(cs);
//!         syst.set_clock_source(SystClkSource::Core);
//!         syst.set_reload(8_000_000); // 1s
//!         syst.enable_counter();
//!         syst.enable_interrupt();
//!     });
//! }
//! 
//! exception!(SYS_TICK, tick);
//! 
//! fn tick() {
//!     interrupt::free(|cs| {
//!         let hstdout = HSTDOUT.borrow(cs);
//!         if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
//!             writeln!(*hstdout, "Tick").ok();
//!         }
//! 
//!         let nvic = stm32f103xx::NVIC.borrow(cs);
//! 
//!         nvic.set_pending(Interrupt::TIM2);
//!     });
//! }
//! 
//! interrupt!(TIM2, tock, locals: {
//!     tocks: u32 = 0;
//! });
//! 
//! fn tock(l: &mut TIM2::Locals) {
//!     l.tocks += 1;
//! 
//!     interrupt::free(|cs| {
//!         let hstdout = HSTDOUT.borrow(cs);
//!         if let Some(hstdout) = hstdout.borrow_mut().as_mut() {
//!             writeln!(*hstdout, "Tock ({})", l.tocks).ok();
//!         }
//!     });
//! }
//! ```
// Auto-generated. Do not modify.
