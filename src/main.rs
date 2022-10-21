#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m::delay::Delay;

use stm32_hal2::{
    clocks::Clocks,
    gpio::{Pin, Port, PinMode, OutputType},
    pac
};


#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    
    let _dp = pac::Peripherals::take().unwrap();

    let clock_cfg = Clocks::default();
    clock_cfg.setup().unwrap();

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, clock_cfg.systick());

    let mut pin5 = Pin::new(Port::A, 5, PinMode::Output);
    pin5.output_type(OutputType::PushPull);
    pin5.set_low();
    pin5.output_speed(stm32_hal2::gpio::OutputSpeed::High);

    let mut set_on: bool = false;

    const DELAY_TIME_MS: u32  = 30;

    loop {
        match set_on {
            false => pin5.set_low(),
            true => pin5.set_high()
        }
        delay.delay_ms(DELAY_TIME_MS);
        set_on = !set_on;
    }
}
