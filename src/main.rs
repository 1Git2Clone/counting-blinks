#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut first_led = pins.d12.into_output();
    let mut second_led = pins.d11.into_output();
    let mut third_led = pins.d10.into_output();
    let mut fourth_led = pins.d9.into_output();

    first_led.set_high();
    second_led.set_high();
    third_led.set_high();
    fourth_led.set_high();

    let mut counter: i32 = 0;

    loop {
        // Set counter data
        if counter >= 15 {
            counter = 0;
        }
        counter += 1;

        // Initialize booleans
        let (mut first, mut second, mut third, mut fourth) = (false, false, false, false);

        // Set booleans based on counter
        match counter {
            c if c % 2 == 1 => {
                first = true;
            }
            c if c % 2 == 0 => {
                first = false;
                second = true;
            }
            c if c % 4 == 0 => {
                second = false;
                third = true;
            }
            c if c % 8 == 0 => {
                third = false;
                fourth = true;
            }
            _ => {}
        }

        // Set all the pin states accordingly. Yes, I know it's done inefficiently but making a
        // closure or a vector isn't achievable in this environment so deal with it.
        match first {
            true => first_led.set_high(),
            false => first_led.set_low(),
        }
        match second {
            true => second_led.set_high(),
            false => second_led.set_low(),
        }
        match third {
            true => third_led.set_high(),
            false => third_led.set_low(),
        }
        match fourth {
            true => fourth_led.set_high(),
            false => fourth_led.set_low(),
        }

        // Wait before iterating again.
        arduino_hal::delay_ms(1000);
    }
}
