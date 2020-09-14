#![no_std]
#![no_main]

use panic_halt as _;

use stm32f4xx_hal::{
    gpio::{self, Edge, ExtiPin},
    prelude::*,
};

use rtic::app;

#[app(device = stm32f4xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        button: gpio::gpioc::PC13<gpio::Input<gpio::PullUp>>,
        led: gpio::gpioa::PA5<gpio::Output<gpio::PushPull>>,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
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

        init::LateResources { led, button }
    }

    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = EXTI15_10, resources = [led, button])]
    fn on_button_press(ctx: on_button_press::Context) {
        let on_button_press::Resources { led, button } = ctx.resources;

        // Clear the interrupt
        button.clear_interrupt_pending_bit();
        // Toggle the led
        led.toggle().ok();
    }
};
