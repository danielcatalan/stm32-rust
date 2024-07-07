#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use stm32f4xx_hal::{pac, prelude::*, rcc::RccExt};
use rtt_target::{debug_rprintln, debug_rtt_init_print};


struct Led<Pin> {
    pin: Pin,
    state: bool,
}

impl<Pin: OutputPin> Led<Pin> {
    pub fn new(pin: Pin) -> Self {
        Led { pin, state: false }
    }

    pub fn toggle(&mut self) {
        self.state = !self.state;
        match self.state {
            true => {
                let _ = self.pin.set_high();
            }
            false => {
                let _ = self.pin.set_low();
            }
        }
    }
}

#[entry]
fn main() -> ! {
    debug_rtt_init_print!(); // init Debug RTT

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(180.MHz()).freeze();

    let mut delay = cp.SYST.delay(&clocks);

    let gpioa = dp.GPIOA.split();
    let pin = gpioa.pa5.into_push_pull_output();
    let mut led = Led::new(pin);

    const DELAY_TIME_MS: u32 = 500;

    loop {
        led.toggle();
        debug_rprintln!("Led toggled!");
        delay.delay_ms(DELAY_TIME_MS);
    }
}
