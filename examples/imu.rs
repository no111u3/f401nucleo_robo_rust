#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::peripheral::Peripherals;

use cortex_m_rt::entry;

use stm32f4xx_hal::{
    delay::Delay,
    i2c::I2c,
    prelude::*,
    serial::{config::Config, Serial},
    stm32,
};

use core::fmt::Write;

use lis3mdl::Lis3mdl;

#[entry]
fn main() -> ! {
    let cp = Peripherals::take().unwrap();

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

    let gpiob = p.GPIOB.split();
    let scl = gpiob
        .pb8
        .into_alternate_af4()
        .internal_pull_up(true)
        .set_open_drain();

    let sda = gpiob
        .pb9
        .into_alternate_af4()
        .internal_pull_up(true)
        .set_open_drain();

    let i2c = I2c::i2c1(p.I2C1, (scl, sda), 200.khz(), clocks);

    writeln!(tx, "IMU sensors from Nucleo F401RE\r").ok();

    // Initialize the LIS3MDL with the I2C
    let mut lis3mdl_device = Lis3mdl::new(i2c).unwrap();

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led.set_high().ok();
        // Read the X, Y, and Z axes values in milliGauss
        let xyz = lis3mdl_device.get_mag_axes_mgauss().unwrap();
        writeln!(tx, "MAG: {{x: {}, y: {}, z: {}}}\r", xyz.x, xyz.y, xyz.z).ok();
        led.set_low().ok();
        delay.delay_ms(100u16);
    }
}
