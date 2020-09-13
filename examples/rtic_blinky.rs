#![no_std]
#![no_main]

use panic_halt as _;

use stm32f4xx_hal::{
    gpio::{self},
    prelude::*,
};

use cortex_m::peripheral::DWT;

use rtic::{app, cyccnt::U32Ext};

const PERIOD: u32 = 8_000_000;

#[app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: gpio::gpioa::PA5<gpio::Output<gpio::PushPull>>,
    }

    #[init(schedule = [blinky])]
    fn init(mut ctx: init::Context) -> init::LateResources {
        let p = ctx.device;

        let gpioa = p.GPIOA.split();

        let led = gpioa.pa5.into_push_pull_output();

        let rcc = p.RCC.constrain();

        let _clocks = rcc.cfgr.freeze();

        // Initialize (enable) the monotonic timer (CYCCNT)
        ctx.core.DCB.enable_trace();

        // required on Cortex-M4 devices that software lock the DWT (e.g. STM32F4)
        DWT::unlock();
        ctx.core.DWT.enable_cycle_counter();

        ctx.schedule.blinky(ctx.start + PERIOD.cycles()).unwrap();

        init::LateResources { led }
    }

    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        loop {}
    }

    #[task(schedule = [blinky], resources = [led])]
    fn blinky(ctx: blinky::Context) {
        let blinky::Resources { led } = ctx.resources;

        // Toggle the led
        led.toggle().ok();

        ctx.schedule
            .blinky(ctx.scheduled + PERIOD.cycles())
            .unwrap();
    }

    extern "C" {
        fn EXTI15_10();
    }
};
