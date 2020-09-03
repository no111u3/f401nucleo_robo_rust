#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use stm32f4xx_hal as _;

#[entry]
fn main() -> ! {
    loop {}
}
