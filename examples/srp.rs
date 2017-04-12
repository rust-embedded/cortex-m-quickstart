#![feature(const_fn)]
#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_srp as rtfm;
extern crate {{name}};

use rtfm::{C2, C4, C16, P0, P1, P3, Resource};
use {{name}}::interrupt::{Exti0Irq, Exti1Irq};

static R1: Resource<(), C4> = Resource::new(());
static R2: Resource<(), C2> = Resource::new(());

fn init(_ceil: C16) {}

fn idle(_prio: P0) {
    hprintln!("IDLE");

    rtfm::request(j1);
}

tasks!({{name}}, {
    j1: (Exti0Irq, P1),
    j2: (Exti1Irq, P3),
});

fn j1(_task: Exti0Irq, prio: P1) {
    hprintln!("J1: enter");
    R2.lock(
        &prio, |_, _| {
            rtfm::request(j2);
            hprintln!("J1: after requesting J2");
            R1.lock(
                &prio, |_, _| {
                    hprintln!("J1(R1): before requesting J2");
                    rtfm::request(j2);
                    hprintln!("J1(R1): after requesting J2");
                }
            );
        }
    );
    hprintln!("J1: exit");
}

fn j2(_task: Exti1Irq, prio: P3) {
    hprintln!("J2: enter");
    R1.lock(
        &prio, |_, _| {
            hprintln!("J2(R1)");
        }
    );
    hprintln!("J2: exit");
}
