#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f4 as _;

#[entry]
fn main() -> ! {
    loop {}
}
