#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use cortex_m::peripheral::Peripherals;

use stm32f4xx_hal::{delay::Delay, prelude::*, pwm, stm32};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let cp = Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();
    let gpioc = p.GPIOC.split();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_low().ok();

    let channels = (
        gpioc.pc6.into_alternate_af2(),
        gpiob.pb5.into_alternate_af2(),
        gpiob.pb0.into_alternate_af2(),
        gpiob.pb1.into_alternate_af2(),
    );

    let (mut ch1, mut ch2, mut ch3, mut ch4) = pwm::tim3(p.TIM3, channels, clocks, 20u32.khz());
    ch1.set_duty(0);
    ch1.enable();
    ch2.set_duty(0);
    ch2.enable();
    ch3.set_duty(0);
    ch3.enable();
    ch4.set_duty(0);
    ch4.enable();

    let mut delay = Delay::new(cp.SYST, clocks);

    let max_duty = ch1.get_max_duty() as u32;

    loop {
        for i in 0..=max_duty * 8 {
            if i < max_duty {
                ch1.set_duty(i as u16);
            } else if i > max_duty - 1 && i < max_duty * 2 {
                ch1.set_duty((max_duty * 2 - i) as u16);
            } else if i > max_duty * 2 - 1 && i < max_duty * 3 {
                ch2.set_duty((i - max_duty * 2) as u16);
            } else if i > max_duty * 3 - 1 && i < max_duty * 4 {
                ch2.set_duty((max_duty * 4 - i) as u16);
            } else if i > max_duty * 4 - 1 && i < max_duty * 5 {
                ch3.set_duty((i - max_duty * 4) as u16);
            } else if i > max_duty * 5 - 1 && i < max_duty * 6 {
                ch3.set_duty((max_duty * 6 - i) as u16);
            } else if i > max_duty * 6 - 1 && i < max_duty * 7 {
                ch4.set_duty((i - max_duty * 6) as u16);
            } else {
                ch4.set_duty((max_duty * 8 - i) as u16);
            }
            delay.delay_ms(2_u16);
        }
        led.toggle().ok();
    }
}
