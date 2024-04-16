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
        match counter & 1 {
            0 => first_led.set_low(),
            1 => first_led.set_high(),
            _ => {
                panic!("You shouldn't have a bitwise operation with the index return something other than 0 or 1");
            }
        }
        match (counter >> 1) & 1 {
            0 => second_led.set_low(),
            1 => second_led.set_high(),
            _ => {
                panic!("You shouldn't have a bitwise operation with the index return something other than 0 or 1");
            }
        }
        match (counter >> 2) & 1 {
            0 => third_led.set_low(),
            1 => third_led.set_high(),
            _ => {
                panic!("You shouldn't have a bitwise operation with the index return something other than 0 or 1");
            }
        }

        // Wait before iterating again.
        arduino_hal::delay_ms(1000);
    }
}
