#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use cortex_m::peripheral::Peripherals;

use stm32f4xx_hal::{
    adc::{config::AdcConfig, config::SampleTime, Adc},
    delay::Delay,
    prelude::*,
    pwm, stm32,
};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let cp = Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_low().ok();

    let channels = gpiob.pb0.into_alternate_af2();

    let mut ch3 = pwm::tim3(p.TIM3, channels, clocks, 20u32.khz());

    ch3.set_duty(0);
    ch3.enable();

    let mut delay = Delay::new(cp.SYST, clocks);

    let max_duty = ch3.get_max_duty() as u32;

    let item_per_millivolts = 3300 / max_duty;

    let mut adc = Adc::adc1(p.ADC1, true, AdcConfig::default());
    let pa0 = gpioa.pa0.into_analog();

    loop {
        let sample = adc.convert(&pa0, SampleTime::Cycles_480);
        let millivolts = adc.sample_to_millivolts(sample);

        ch3.set_duty(millivolts / item_per_millivolts as u16);

        delay.delay_ms(20u16);

        led.toggle().ok();
    }
}
