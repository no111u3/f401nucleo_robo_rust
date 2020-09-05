#![no_std]
#![no_main]

use core::cell::RefCell;

extern crate panic_halt;

use cortex_m_rt::entry;

use cortex_m::{interrupt::Mutex, peripheral::Peripherals};

use stm32f4xx_hal::{
    gpio, interrupt,
    prelude::*,
    stm32,
    timer::{Event, Timer},
};

static TIMER: Mutex<RefCell<Option<Timer<stm32::TIM2>>>> = Mutex::new(RefCell::new(None));
static LEDS: Mutex<RefCell<Option<[gpio::gpioa::PA<gpio::Output<gpio::PushPull>>; 8]>>> =
    Mutex::new(RefCell::new(None));
static COUNTER: Mutex<RefCell<Option<usize>>> = Mutex::new(RefCell::new(Some(0)));

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let _cp = Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let gpioc = p.GPIOC.split();

    let button = gpioc.pc13.into_pull_up_input();

    let leds = [
        gpioa.pa0.into_push_pull_output().downgrade(),
        gpioa.pa1.into_push_pull_output().downgrade(),
        gpioa.pa10.into_push_pull_output().downgrade(),
        gpioa.pa8.into_push_pull_output().downgrade(),
        gpioa.pa9.into_push_pull_output().downgrade(),
        gpioa.pa7.into_push_pull_output().downgrade(),
        gpioa.pa6.into_push_pull_output().downgrade(),
        gpioa.pa5.into_push_pull_output().downgrade(),
    ];

    cortex_m::interrupt::free(|cs| {
        LEDS.borrow(cs).replace(Some(leds));
    });

    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    // Setup timer
    let mut timer = Timer::tim2(p.TIM2, 4.hz(), clocks);

    // Enable interrupt
    timer.listen(Event::TimeOut);

    cortex_m::interrupt::free(|cs| {
        TIMER.borrow(cs).replace(Some(timer));
    });
    // Enable TIM2 interrupt
    unsafe { cortex_m::peripheral::NVIC::unmask(stm32::Interrupt::TIM2) }

    loop {
        if button.is_low().unwrap() {
            cortex_m::interrupt::free(|cs| {
                let mut timer = TIMER.borrow(cs).borrow_mut();

                timer.as_mut().unwrap().listen(Event::TimeOut);
            })
        } else {
            cortex_m::interrupt::free(|cs| {
                let mut timer = TIMER.borrow(cs).borrow_mut();
                let mut counter = COUNTER.borrow(cs).borrow_mut();
                let mut leds = LEDS.borrow(cs).borrow_mut();

                timer.as_mut().unwrap().unlisten(Event::TimeOut);

                *counter.as_mut().unwrap() = 0;

                for led in leds.as_mut().unwrap().iter_mut() {
                    led.set_low().ok();
                }
            });
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
        let mut leds = LEDS.borrow(cs).borrow_mut();
        let mut counter = COUNTER.borrow(cs).borrow_mut();
        let mut led_num = *counter.as_ref().unwrap();

        let leds = leds.as_mut().unwrap();
        leds[led_num].set_high().ok();
        leds[if led_num > 0 {
            led_num - 1
        } else {
            leds.len() - 1
        }]
        .set_low()
        .ok();
        led_num = if led_num < leds.len() - 1 {
            led_num + 1
        } else {
            0
        };
        *counter.as_mut().unwrap() = led_num;
    });
}
