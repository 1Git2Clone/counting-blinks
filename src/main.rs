#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut first_led = (0u8, pins.d10.into_output());
    let mut second_led = (1u8, pins.d11.into_output());
    let mut third_led = (2u8, pins.d12.into_output());

    first_led.1.set_low();
    second_led.1.set_low();
    third_led.1.set_low();

    let mut counter: u8 = 0;

    loop {
        // Set counter data
        counter += 1;
        if counter > 0b111 {
            counter = 1;
        }

        // Match bitwise operations on the leds using their binary index.
        match (counter >> first_led.0) & 1 {
            0 => first_led.1.set_low(),
            1 => first_led.1.set_high(),
            _ => {
                panic!("You shouldn't have a bitwise operation with the index return something other than 0 or 1");
            }
        }
        match (counter >> second_led.0) & 1 {
            0 => second_led.1.set_low(),
            1 => second_led.1.set_high(),
            _ => {
                panic!("You shouldn't have a bitwise operation with the index return something other than 0 or 1");
            }
        }
        match (counter >> third_led.0) & 1 {
            0 => third_led.1.set_low(),
            1 => third_led.1.set_high(),
            _ => {
                panic!("You shouldn't have a bitwise operation with the index return something other than 0 or 1");
            }
        }

        // Initialize booleans

        // Wait before iterating again.
        arduino_hal::delay_ms(1000);
    }
}
