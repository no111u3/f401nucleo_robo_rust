#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use cortex_m::peripheral::Peripherals;

use stm32f4xx_hal::{delay::Delay, gpio, prelude::*, stm32};

const SEG_A: u8 = 1u8 << 0;
const SEG_B: u8 = 1u8 << 1;
const SEG_C: u8 = 1u8 << 2;
const SEG_D: u8 = 1u8 << 3;
const SEG_E: u8 = 1u8 << 4;
const SEG_F: u8 = 1u8 << 5;
const SEG_G: u8 = 1u8 << 6;
const SEG_DP: u8 = 1u8 << 7;

const NUMS: [u8; 11] = [
    SEG_A | SEG_B | SEG_C | SEG_D | SEG_E | SEG_F, // 0
    SEG_B | SEG_C,                                 // 1
    SEG_A | SEG_B | SEG_G | SEG_E | SEG_D,         // 2
    SEG_A | SEG_B | SEG_G | SEG_C | SEG_D,         // 3
    SEG_F | SEG_G | SEG_B | SEG_C,                 // 4
    SEG_A | SEG_F | SEG_G | SEG_C | SEG_D,         // 5
    SEG_F | SEG_G | SEG_C | SEG_D | SEG_E,         // 6
    SEG_A | SEG_B | SEG_C,                         // 7
    SEG_A | SEG_B | SEG_C | SEG_D | SEG_E | SEG_F | SEG_G, // 8
    SEG_F | SEG_A | SEG_B | SEG_G | SEG_C,         // 9
    SEG_DP,                                        // Decimal point
];

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
        gpioa.pa4.into_push_pull_output().downgrade(),
        gpioa.pa5.into_push_pull_output().downgrade(),
        gpioa.pa6.into_push_pull_output().downgrade(),
        gpioa.pa7.into_push_pull_output().downgrade(),
        gpioa.pa8.into_push_pull_output().downgrade(),
        gpioa.pa9.into_push_pull_output().downgrade(),
    ];

    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let mut delay = Delay::new(cp.SYST, clocks);

    for i in 0..leds.len() {
        leds[i].set_high().ok();

        leds[if i < 1 { leds.len() - 1 } else { i - 1 }]
            .set_low()
            .ok();

        delay.delay_ms(250u16);
    }
    leds[leds.len() - 1].set_low().ok();

    let mut button_count: u8 = 0;
    loop {
        if button.is_low().unwrap() {
            if button_count < 10 {
                button_count += 1;
            } else {
                for num in &NUMS {
                    apply_segments(&mut leds, *num);
                    delay.delay_ms(500u16);
                }
            }
        } else {
            button_count = 0;
        }
    }
}

fn apply_segments(leds: &mut [gpio::gpioa::PA<gpio::Output<gpio::PushPull>>; 8], segments: u8) {
    for i in 0..leds.len() {
        if segments & 1u8 << i != 0 {
            leds[i].set_high().ok();
        } else {
            leds[i].set_low().ok();
        }
    }
}
