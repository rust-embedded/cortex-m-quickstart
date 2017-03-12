//! LED blinking
//!
//! This example is specific to the STM32F3DISCOVERY board

#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_srp;
extern crate {{name}};

use core::cell::Cell;

use cortex_m::ctxt::Local;
use cortex_m::exception;
use cortex_m_srp::{C0, ResourceP};
use {{name}}::interrupt::{self, Interrupt};

// INITIALIZATION
fn main() {
    cortex_m::interrupt::free(|cs| {
        let gpioe = {{name}}::GPIOE.borrow(&cs);
        let nvic = cortex_m::peripheral::NVIC.borrow(&cs);
        let rcc = {{name}}::RCC.borrow(&cs);
        let tim7 = {{name}}::TIM7.borrow(&cs);

        // enable peripherals
        rcc.ahbenr.modify(|_, w| unsafe { w.iopeen().bits(1) });
        rcc.apb1enr.modify(|_, w| unsafe { w.tim7en().bits(1) });

        // configure PE8 as output
        gpioe.moder.modify(|_, w| unsafe { w.moder8().bits(0b01) });

        // 1 Hz timeout
        tim7.dier.modify(|_, w| unsafe { w.uie().bits(1) });
        tim7.psc.write(|w| unsafe { w.psc().bits(122) });
        tim7.arr.write(|w| unsafe { w.arr().bits(65040) });
        tim7.cr1.write(|w| unsafe { w.cen().bits(1) });

        nvic.enable(Interrupt::Tim7);
    });
}

// RESOURCES
static GPIOE: ResourceP<{{name}}::Gpioe, C0> =
    unsafe { ResourceP::new({{name}}::GPIOE) };

static TIM7: ResourceP<{{name}}::Tim7, C0> =
    unsafe { ResourceP::new({{name}}::TIM7) };

// TASKS
extern "C" fn blinky(ctxt: interrupt::Tim7) {
    static STATE: Local<Cell<bool>, interrupt::Tim7> =
        Local::new(Cell::new(false));

    let gpioe = GPIOE.borrow(&ctxt);
    let state = STATE.borrow(&ctxt);
    let tim7 = TIM7.borrow(&ctxt);

    // Clear the update flag
    tim7.sr.modify(|_, w| unsafe { w.uif().bits(0) });

    // Toggle the state
    state.set(!state.get());

    // Blink!
    if state.get() {
        gpioe.bsrr.write(|w| unsafe { w.br8().bits(1) });
    } else {
        gpioe.bsrr.write(|w| unsafe { w.bs8().bits(1) });
    }
}

// GLUE
#[no_mangle]
pub static _INTERRUPTS: interrupt::Handlers =
    interrupt::Handlers { tim7: blinky, ..interrupt::DEFAULT_HANDLERS };

#[no_mangle]
pub static _EXCEPTIONS: exception::Handlers =
    exception::Handlers { ..exception::DEFAULT_HANDLERS };
