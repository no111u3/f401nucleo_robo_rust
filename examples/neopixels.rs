#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use cortex_m::peripheral::Peripherals;

use stm32f4xx_hal::{
    delay::Delay,
    prelude::*,
    spi::{self, Spi},
    stm32,
};

use ws2812_spi::{Ws2812, MODE};

use smart_leds::{SmartLedsWrite, White, RGBW};

type Color = RGBW<u8, u8>;

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let cp = Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_low().ok();

    let sck = gpiob.pb3.into_alternate_af5();
    let miso = gpioa.pa6.into_alternate_af5();
    let mosi = gpioa.pa7.into_alternate_af5();

    let mut delay = Delay::new(cp.SYST, clocks);

    let spi = Spi::spi1(p.SPI1, (sck, miso, mosi), MODE, 3_000_000.hz(), clocks);

    let mut ws = Ws2812::new_sk6812w(spi);

    let data = [
        Color {
            r: 10,
            g: 0,
            b: 0,
            a: White(0),
        },
        Color {
            r: 0,
            g: 10,
            b: 0,
            a: White(0),
        },
        Color {
            r: 0,
            g: 0,
            b: 10,
            a: White(0),
        },
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: White(5),
        },
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: White(5),
        },
    ];
    let empty = [Color::default(); 5];

    loop {
        ws.write(data.iter().cloned()).unwrap();
        led.set_high().ok();
        delay.delay_ms(1000u16);
        ws.write(empty.iter().cloned()).unwrap();
        led.set_low().ok();
        delay.delay_ms(1000u16);
    }
}
