# Counting blinks

Rust project for the _Arduino Nano_.

## Prerequisites

Arduino nano.

A breadboard (at least from a1-j30 which should be the smallest one).

4 jumper wires (male to male).

3 resistors (I'm using metal film blue ones).

3 LED diodes (the colors don't matter).

## Example Board Layout

Arduino from d30 to j16.

Jumper cable at GND (in this case: d19 or j17) to negative column at the j column side.

3 Jumper cables:
- d30-a1 (pin.d12)
- d29-a5 (pin.d11)
- d28-a9 (pin.d10)

3 Resistors:
- b1-i9
- b5-i5
- b9-j9

Some vocabulary for the LEDs:
- Anode - the long end (positive charged): the part where oxidization happens.
- Cathode - the short part (negative charged): the part where reduction happens.

3 LEDs:
- Anode: j1 | Cathode: at the negative row
- Anode: j5 | Cathode: at the negative row
- Anode: j9 | Cathode: at the negative row

> [!NOTE]
> The position of the cathodes on the row doesn't matter as long as they're on the row where the GND jump wire is connected.

For reference, it should look something similar to the following:

![Example diagram](https://www.makerguides.com/wp-content/uploads/2024/04/image-33.png)

Showcase vid:

[![](https://github.com/1Kill2Steal/counting-blinks/blob/main/assets/preview-vid.gif)](https://github.com/1Kill2Steal/counting-blinks/blob/main/assets/preview-vid.mp4(

## Code Build Instructions

1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board. If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

At your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
