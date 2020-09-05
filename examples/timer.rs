#![no_std]
#![no_main]

use core::cell::RefCell;

extern crate panic_halt;

use cortex_m_rt::entry;

use cortex_m::{interrupt::Mutex, peripheral::Peripherals};

use stm32f4xx_hal::{
    delay::Delay,
    gpio, interrupt,
    prelude::*,
    stm32,
    timer::{Event, Timer},
};

static TIMER: Mutex<RefCell<Option<Timer<stm32::TIM2>>>> = Mutex::new(RefCell::new(None));
static LED: Mutex<RefCell<Option<gpio::gpioa::PA<gpio::Output<gpio::PushPull>>>>> =
    Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let cp = Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let gpioc = p.GPIOC.split();

    let button = gpioc.pc13.into_pull_up_input();

    let led = gpioa.pa0.into_push_pull_output().downgrade();

    cortex_m::interrupt::free(|cs| {
        LED.borrow(cs).replace(Some(led));
    });

    let mut leds = [
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

    // Setup timer
    let mut timer = Timer::tim2(p.TIM2, 1.hz(), clocks);

    // Enable interrupt
    timer.listen(Event::TimeOut);

    cortex_m::interrupt::free(|cs| {
        TIMER.borrow(cs).replace(Some(timer));
    });

    let mut delay = Delay::new(cp.SYST, clocks);

    // Enable TIM2 interrupt
    unsafe { cortex_m::peripheral::NVIC::unmask(stm32::Interrupt::TIM2) }

    loop {
        if button.is_low().unwrap() {
            for led_num in 0..leds.len() {
                leds[led_num].set_high().ok();
                delay.delay_ms(250_u16);
                leds[if led_num > 0 {
                    led_num - 1
                } else {
                    leds.len() - 1
                }]
                .set_low()
                .ok();
            }
        } else {
            for led in &mut leds {
                led.set_low().ok();
            }
        }
    }
}

#[interrupt]
fn TIM2() {
    // Ack the interrupt
    unsafe {
        (*stm32::TIM2::ptr()).sr.modify(|_, w| w.uif().clear_bit());
    }

    // Toggle led
    cortex_m::interrupt::free(|cs| {
        let mut led = LED.borrow(cs).borrow_mut();
        led.as_mut().unwrap().toggle().ok();
    });
}
