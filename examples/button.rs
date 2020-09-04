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
    let gpioc = p.GPIOC.split();

    let button = gpioc.pc13.into_pull_up_input();

    let mut leds = [
        gpioa.pa0.into_push_pull_output().downgrade(),
        gpioa.pa1.into_push_pull_output().downgrade(),
        gpioa.pa10.into_push_pull_output().downgrade(),
        gpioa.pa8.into_push_pull_output().downgrade(),
        gpioa.pa9.into_push_pull_output().downgrade(),
        gpioa.pa7.into_push_pull_output().downgrade(),
        gpioa.pa6.into_push_pull_output().downgrade(),
        gpioa.pa5.into_push_pull_output().downgrade(),
    ];

    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        if button.is_low().unwrap() {
            for led_num in 0..leds.len() {
                leds[led_num].set_high().ok();
                leds[if led_num > 0 {
                    led_num - 1
                } else {
                    leds.len() - 1
                }]
                .set_low()
                .ok();
                delay.delay_ms(250_u16);
            }
        } else {
            for led in &mut leds {
                led.set_low().ok();
            }
        }
    }
}
