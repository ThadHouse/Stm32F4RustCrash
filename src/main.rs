#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _; // panic handler
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use stm32f4xx_hal as _;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");
    loop {}
}
