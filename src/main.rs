#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut first_led = pins.d10.into_output();
    let mut second_led = pins.d11.into_output();
    let mut third_led = pins.d12.into_output();

    first_led.set_low();
    second_led.set_low();
    third_led.set_low();

    let mut counter: u8 = 0;

    loop {
        // Set counter data
        counter += 1;
        if counter > 0b111 {
            counter = 1;
        }

        // Match bitwise operations on the leds using their binary index.
        match (counter & 1) == 0 {
            true => first_led.set_low(),
            false => first_led.set_high(),
        }
        match ((counter >> 1) & 1) == 0 {
            true => second_led.set_low(),
            false => second_led.set_high(),
        }
        match ((counter >> 2) & 1) == 0 {
            true => third_led.set_low(),
            false => third_led.set_high(),
        }

        // Wait before iterating again.
        arduino_hal::delay_ms(1000);
    }
}
