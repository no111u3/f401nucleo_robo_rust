#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use stm32f4xx_hal::{
    prelude::*,
    serial::{config::Config, Serial},
    stm32,
};

use core::fmt::Write;

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_low().ok();

    let tx = gpioa.pa2.into_alternate_af7();
    let rx = gpioa.pa3.into_alternate_af7();

    let config = Config::default().baudrate(4_800.bps());

    let serial = Serial::usart2(p.USART2, (tx, rx), config, clocks).unwrap();

    let (mut tx, _rx) = serial.split();

    writeln!(tx, "Float operations from Nucleo F401RE\r").ok();

    writeln!(tx, "Simple operations\r").ok();
    let x = 1.5f32;
    let k = 4.2f32;
    let b = 8.3f32;

    let y = k * x + b;
    writeln!(tx, "{} * {} + {} = {}\r", k, x, b, y).ok();

    let t = 0.123;
    let mut m = 1.0;
    let mut x = 0.0;

    writeln!(tx, "MLAC operations\r").ok();

    for i in 0..100 {
        m *= t;
        x += m;
        writeln!(tx, "for i = {} m = {}, x = {}\r", i, m, x).ok();
    }

    loop {}
}
