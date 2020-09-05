#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use stm32f4xx_hal::{
    nb::block,
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

    let (mut tx, mut rx) = serial.split();

    writeln!(tx, "Serial echo from Nucleo F401RE\r").ok();

    loop {
        let received = block!(rx.read()).unwrap();
        led.set_high().ok();
        block!(tx.write(received)).ok();
        led.set_low().ok();
    }
}
