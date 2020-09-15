#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use stm32f4xx_hal::{
    adc::{config::AdcConfig, config::SampleTime, Adc},
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

    writeln!(tx, "ADC from Nucleo F401RE\r").ok();

    let mut adc = Adc::adc1(p.ADC1, true, AdcConfig::default());
    let pa0 = gpioa.pa0.into_analog();

    loop {
        let sample = adc.convert(&pa0, SampleTime::Cycles_480);
        let millivolts = adc.sample_to_millivolts(sample);
        writeln!(tx, "Channel0: {}mv\r", millivolts).ok();
    }
}
