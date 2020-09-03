#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use cortex_m::peripheral::Peripherals;

use stm32f4xx_hal::{delay::Delay, prelude::*, stm32};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let cp = Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();

    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_low().ok();

    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        delay.delay_ms(500_u16);
        led.toggle().ok();
    }
}
