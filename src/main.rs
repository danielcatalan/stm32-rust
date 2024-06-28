#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use cortex_m_rt::entry;
use stm32f4xx_hal::{gpio::PinState, pac, prelude::*, rcc::RccExt};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(180.MHz()).freeze();


    let mut delay = cp.SYST.delay(&clocks);

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    let mut set_on: bool = false;

    const DELAY_TIME_MS: u32 = 1000;

    loop {
        match set_on {
            true => led.set_state(PinState::High),
            false => led.set_state(PinState::Low),
        }

        delay.delay_ms(DELAY_TIME_MS);
        set_on = !set_on;
    }
}
