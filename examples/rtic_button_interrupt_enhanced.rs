#![no_std]
#![no_main]

use panic_halt as _;

use stm32f4xx_hal::{
    gpio::{self, Edge, ExtiPin},
    prelude::*,
};

use cortex_m::peripheral::DWT;

use rtic::{app, cyccnt::U32Ext};

#[derive(PartialEq)]
pub enum Event {
    Released,
    Pressed,
    OnPress,
    OnRelease,
}

#[app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        button: gpio::gpioc::PC13<gpio::Input<gpio::PullUp>>,
        led: gpio::gpioa::PA5<gpio::Output<gpio::PushPull>>,
        event: Event,
        counter: u32,
    }

    #[init]
    fn init(mut ctx: init::Context) -> init::LateResources {
        let mut p = ctx.device;

        let gpioa = p.GPIOA.split();
        let gpioc = p.GPIOC.split();

        let mut button = gpioc.pc13.into_pull_up_input();

        let led = gpioa.pa5.into_push_pull_output();

        p.RCC.apb2enr.modify(|_, w| w.syscfgen().enabled());

        let rcc = p.RCC.constrain();

        let _clocks = rcc.cfgr.freeze();

        button.make_interrupt_source(&mut p.SYSCFG);
        button.enable_interrupt(&mut p.EXTI);
        button.trigger_on_edge(&mut p.EXTI, Edge::RISING);

        // Initialize (enable) the monotonic timer (CYCCNT)
        ctx.core.DCB.enable_trace();

        // required on Cortex-M4 devices that software lock the DWT (e.g. STM32F4)
        DWT::unlock();
        ctx.core.DWT.enable_cycle_counter();

        let event = Event::Released;

        let counter = 0;

        init::LateResources {
            led,
            button,
            event,
            counter,
        }
    }

    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(schedule = [button_handler], resources = [led, button, event, counter])]
    fn button_handler(ctx: button_handler::Context) {
        let button_handler::Resources {
            led,
            button,
            event,
            counter,
        } = ctx.resources;

        match event {
            Event::OnPress => {
                if *counter < 21 {
                    if button.is_low().unwrap() {
                        *counter += 1;
                        ctx.schedule
                            .button_handler(ctx.scheduled + 16_000.cycles())
                            .unwrap();
                    } else {
                        *event = Event::OnRelease;
                        ctx.schedule.button_handler(ctx.scheduled).unwrap();
                    }
                } else {
                    *event = Event::Pressed;
                    ctx.schedule.button_handler(ctx.scheduled).unwrap();
                }
            }
            Event::OnRelease => {
                *event = Event::Released;
                *counter = 0;
                // Clear the interrupt
                button.clear_interrupt_pending_bit();
            }
            Event::Released => {
                if button.is_low().unwrap() {
                    *event = Event::OnPress;
                    ctx.schedule
                        .button_handler(ctx.scheduled + 16_000.cycles())
                        .unwrap();
                }
            }
            Event::Pressed => {
                // Toggle the led
                led.toggle().ok();
                if button.is_low().unwrap() {
                    ctx.schedule
                        .button_handler(ctx.scheduled + 16_000.cycles())
                        .unwrap();
                } else {
                    *event = Event::OnRelease;
                    ctx.schedule.button_handler(ctx.scheduled).unwrap();
                }
            }
        }
    }

    #[task(binds = EXTI15_10, schedule = [button_handler], resources = [button, event])]
    fn on_button_press(ctx: on_button_press::Context) {
        let on_button_press::Resources { button, event } = ctx.resources;

        if *event == Event::Released && button.is_low().unwrap() {
            *event = Event::OnPress;
            ctx.schedule
                .button_handler(ctx.start + 16_000.cycles())
                .unwrap();
        }
    }

    extern "C" {
        fn EXTI9_5();
    }
};
