#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

fn device_init() {
    // power clock enable
    let periph_base = 0x40000000u32;
    let ahb1periph_base = periph_base + 0x00020000u32;
    let rcc_base = ahb1periph_base + 0x3800u32;
    let rcc_ahb1enr = rcc_base + 0x30;

    let rcc_apb1enr_pwren =  1u32 << 0u32;
    unsafe { core::ptr::write_volatile(rcc_ahb1enr as *mut u32, rcc_apb1enr_pwren) }


}

fn gpio_init() {
    // set PA5 to output, push/pull, pull-up, low-speed

    // PortA is at 0x40020000
    let port_addr: u32 = 0x4002_0000;
    let pin = 5;
    let pin_mask = 1 << pin;

    // Reset pin
    let bsrr_addr = port_addr + 0x18;
    let bsrr_value = pin_mask << 16;
    // let bsrr_value = pin_mask ;
    unsafe { core::ptr::write_volatile(bsrr_addr as *mut u32, bsrr_value) }

    // Set speed
    let low_speed_value = 0x00;
    let ospeedr_addr = port_addr + 0x08;
    let ospeedr_value = low_speed_value << (pin * 2);
    unsafe { core::ptr::write_volatile(ospeedr_addr as *mut u32, ospeedr_value) }

    // Set output type
    let otyper_value = 0x00;
    let otyper_addr = port_addr + 0x04;
    unsafe { core::ptr::write_volatile(otyper_addr as *mut u32, otyper_value) }

    // Set pullup/pulldown
    let pupdr_addr = port_addr + 0x0C;
    let mut pupdr_value: u32;
    unsafe {
        pupdr_value = core::ptr::read_volatile(pupdr_addr as *mut u32);
    }
    let pin5_mask: u32 = 3 << 10;
    pupdr_value = (pupdr_value & (!pin5_mask)) | (0x1 << 10);
    unsafe { core::ptr::write_volatile(pupdr_addr as *mut u32, pupdr_value) }

    // Set as output
    let moder_addr = port_addr + 0x00;
    let mut moder_value: u32;
    unsafe {
        moder_value = core::ptr::read_volatile(moder_addr as *mut u32);
    }
    moder_value = moder_value & !(3 << 10u32); // reset mode for pin5
    moder_value = moder_value | (1 << 10u32);
    unsafe { core::ptr::write_volatile(moder_addr as *mut u32, moder_value) }

    // set alt
    let afrl_addr = port_addr + 0x20;
    unsafe { core::ptr::write_volatile(afrl_addr as *mut u32, 0x00) }


}

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    device_init();

    gpio_init();
    let port_addr: u32 = 0x4002_0000;
    let bsrr_addr = port_addr + 0x18;

    let mut on :bool = true;

    loop {
        on = !on;
        let pin5_hi = set_pin(on);
        unsafe { core::ptr::write_volatile(bsrr_addr as *mut u32, pin5_hi) }

        for _ in 0..20000{
            asm::nop();
        }

    }
}

fn set_pin(on: bool) -> u32{
    match on{
        true => 1<<5,
        false => 1<<(5+16)
    }
}