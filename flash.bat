openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "reset halt" -c "flash write_image erase target/thumbv7em-none-eabihf/debug/blink" -c "reset run" -c exit
