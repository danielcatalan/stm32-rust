#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;


fn gpio_init()
{

    // set PA5 to output, push/pull, pull-up

    // PortA is at 0x40020000
    let port_addr = 0x4002_0000;
    let pin = 0x20;

    let bsrr_arr = port_addr + 0x18;
    let bsrr_value = pin << 16;

    // Reset pin
    unsafe{
        core::ptr::write_volatile(bsrr_arr as *mut u32, bsrr_value)
    }

}

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    gpio_init();

    loop {
        // your code goes here
    }
}
