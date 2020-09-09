#![no_std]
#![no_main]

use panic_halt as _;

use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
};

use cortex_m_rt::entry;

use cortex_m::{interrupt::Mutex, peripheral::Peripherals};

use stm32f4xx_hal::{
    gpio::{self, Edge, ExtiPin},
    interrupt,
    prelude::*,
    stm32,
};

static SIGNAL: AtomicBool = AtomicBool::new(false);

static BUTTON: Mutex<RefCell<Option<gpio::gpioc::PC13<gpio::Input<gpio::PullUp>>>>> =
    Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let mut p = stm32::Peripherals::take().unwrap();

    let _cp = Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let gpioc = p.GPIOC.split();

    let mut button = gpioc.pc13.into_pull_up_input();

    let mut led = gpioa.pa5.into_push_pull_output();

    p.RCC.apb2enr.modify(|_, w| w.syscfgen().enabled());

    let rcc = p.RCC.constrain();

    let _clocks = rcc.cfgr.freeze();

    button.make_interrupt_source(&mut p.SYSCFG);
    button.enable_interrupt(&mut p.EXTI);
    button.trigger_on_edge(&mut p.EXTI, Edge::RISING);

    cortex_m::interrupt::free(|cs| {
        BUTTON.borrow(cs).replace(Some(button));
    });

    // Enable the external interrupt
    unsafe {
        cortex_m::peripheral::NVIC::unmask(stm32::Interrupt::EXTI15_10);
    }

    loop {
        let state_change = SIGNAL.load(Ordering::SeqCst);
        if state_change {
            led.toggle().ok();
            SIGNAL.store(false, Ordering::SeqCst);
        }
    }
}

#[interrupt]
fn EXTI15_10() {
    // Clear the interrupt
    cortex_m::interrupt::free(|cs| {
        let mut button = BUTTON.borrow(cs).borrow_mut();
        button.as_mut().unwrap().clear_interrupt_pending_bit();
    });

    SIGNAL.store(true, Ordering::SeqCst);
}
