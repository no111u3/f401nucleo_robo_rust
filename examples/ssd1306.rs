#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use cortex_m::peripheral::Peripherals;

use stm32f4xx_hal::{
    delay::Delay,
    prelude::*,
    spi::{self, Spi},
    stm32,
};

use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::PrimitiveStyleBuilder,
};

use ssd1306::{prelude::*, Builder as SSD1306Builder};

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

    let sck = gpiob.pb3.into_alternate_af5();
    let miso = spi::NoMiso;
    let mosi = gpioa.pa7.into_alternate_af5();

    let dc = gpiob.pb4.into_push_pull_output();
    let cs = gpioa.pa10.into_push_pull_output();
    let mut res = gpiob.pb5.into_push_pull_output();

    let mut delay = Delay::new(cp.SYST, clocks);

    let mode = spi::Mode {
        polarity: spi::Polarity::IdleLow,
        phase: spi::Phase::CaptureOnFirstTransition,
    };

    let spi = Spi::spi1(p.SPI1, (sck, miso, mosi), mode, 4_000_000.hz(), clocks);

    let iface = SPIInterface::new(spi, dc, cs);

    let mut disp: GraphicsMode<_> = SSD1306Builder::new().connect(iface).into();
    disp.reset(&mut res, &mut delay).unwrap();
    disp.init().unwrap();
    disp.flush().unwrap();

    let yoffset = 20;

    loop {
        delay.delay_ms(500_u16);
        led.toggle().ok();

        disp.clear();
        let style = PrimitiveStyleBuilder::new()
            .stroke_width(1)
            .stroke_color(BinaryColor::On)
            .build();

        // screen outline
        // default display size is 128x64 if you don't pass a _DisplaySize_
        // enum to the _Builder_ struct
        Rectangle::new(Point::new(0, 0), Point::new(127, 63))
            .into_styled(style)
            .draw(&mut disp)
            .unwrap();

        // triangle
        Triangle::new(
            Point::new(16, 16 + yoffset),
            Point::new(16 + 16, 16 + yoffset),
            Point::new(16 + 8, yoffset),
        )
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

        // square
        Rectangle::new(Point::new(52, yoffset), Point::new(52 + 16, 16 + yoffset))
            .into_styled(style)
            .draw(&mut disp)
            .unwrap();

        // circle
        Circle::new(Point::new(96, yoffset + 8), 8)
            .into_styled(style)
            .draw(&mut disp)
            .unwrap();

        disp.flush().unwrap();

        delay.delay_ms(500_u16);
        led.toggle().ok();

        disp.clear();

        let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64, 64);

        let im = Image::new(&raw, Point::new(32, 0));

        im.draw(&mut disp).unwrap();

        disp.flush().unwrap();
    }
}
