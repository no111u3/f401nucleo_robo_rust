#![no_std]
#![no_main]

use panic_halt as _;

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
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(32.mhz()).freeze();

    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_low().ok();

    let sck = spi::NoSck;
    let miso = spi::NoMiso;
    let mosi = gpioa.pa7.into_alternate_af5();

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut spi = Spi::spi1(p.SPI1, (sck, miso, mosi), MODE, 4_000_000.hz(), clocks);
    spi.set_send_only();

    const LED_NUM: usize = 10;
    const COLOR1: Color = Color {
        r: 0x00,
        g: 0xc3 / 5,
        b: 0x36 / 5,
        a: White(0x00),
    };
    const COLOR2: Color = Color {
        r: 0x00,
        g: 0x24 / 5,
        b: 0xb0 / 5,
        a: White(0x00),
    };

    let mut data = [Color::default(); LED_NUM];
    let mut main = 0;
    let mut up = true;

    let mut ws = Ws2812::new_sk6812w(spi);

    loop {
        for i in 0..LED_NUM {
            let distance = (main as i32 - i as i32).abs() as u8;
            let c1 = (
                COLOR1.r as u32 * (LED_NUM as u32 - distance as u32) / LED_NUM as u32,
                COLOR1.g as u32 * (LED_NUM as u32 - distance as u32) / LED_NUM as u32,
                COLOR1.b as u32 * (LED_NUM as u32 - distance as u32) / LED_NUM as u32,
            );
            let c2 = (
                COLOR2.r as u32 * distance as u32 / LED_NUM as u32,
                COLOR2.g as u32 * distance as u32 / LED_NUM as u32,
                COLOR2.b as u32 * distance as u32 / LED_NUM as u32,
            );
            let ct = (
                (c1.0 + c2.0) as u8,
                (c1.1 + c2.1) as u8,
                (c1.2 + c2.2) as u8,
                White(0u8),
            )
                .into();
            data[i] = ct;
        }
        if up {
            if main == LED_NUM - 1 {
                up = false;
                main -= 2;
            }
            main += 1;
        } else {
            if main == 0 {
                up = true;
                main += 2;
            }
            main -= 1;
        }
        ws.write(data.iter().cloned()).unwrap();
        led.set_high().ok();
        delay.delay_ms(100u16);
        led.set_low().ok();
        delay.delay_ms(100u16);
    }
}
