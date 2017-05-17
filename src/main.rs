#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;

extern crate f3;

use f3::led::{self, LEDS};
use f3::stm32f30x;
use f3::stm32f30x::interrupt::Tim7;
use f3::timer::Timer;

use rtfm::{Local, P0, P1, T0, T1, TMax};

peripherals!(stm32f30x, {
    GPIOE: Peripheral {
        register_block: Gpioe,
        ceiling: C0,
    },
    RCC: Peripheral {
        register_block: Rcc,
        ceiling: C0,
    },
    TIM7: Peripheral {
        register_block: Tim7,
        ceiling: C1,
    },
});

fn init(ref priority: P0, threshold: &TMax) {
    let gpioe = GPIOE.access(priority, threshold);
    let rcc = RCC.access(priority, threshold);
    let tim7 = TIM7.access(priority, threshold);

    let timer = Timer(&tim7);

    led::init(&gpioe, &rcc);
    timer.init(&rcc, 8);
    timer.resume();
}

fn idle(_priority: P0, _threshold: T0) -> ! {
    loop {
        rtfm::wfi();
    }
}

tasks!(stm32f30x, {
    roulette: Task {
        interrupt: Tim7,
        priority: P1,
        enabled: true,
    },
});

fn roulette(mut task: Tim7, ref priority: P1, ref threshold: T1) {
    static STATE: Local<u8, Tim7> = Local::new(0);

    let tim7 = TIM7.access(priority, threshold);
    let timer = Timer(&tim7);

    if timer.clear_update_flag().is_ok() {
        let state = STATE.borrow_mut(&mut task);
        let curr = *state;
        let next = (curr + 1) % LEDS.len() as u8;

        LEDS[curr as usize].off();
        LEDS[next as usize].on();

        *state = next;
    }
}
