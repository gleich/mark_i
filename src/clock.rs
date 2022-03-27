use std::{thread, time::Duration};

use is31fl3731::{devices::CharlieBonnet, IS31FL3731};
use rppal::{
    hal,
    i2c::{self, I2c},
};

pub fn setup(i2c: I2c) -> IS31FL3731<i2c::I2c> {
    let mut delay = hal::Delay;
    let mut ic = CharlieBonnet::configure(i2c);
    println!("Animating display on");
    ic.setup(&mut delay).expect("Failed to setup display");
    for x in 0..16 {
        for y in 0..8 {
            ic.pixel(x, y, 1).expect("Failed to set pixel value");
            thread::sleep(Duration::from_millis(20));
        }
    }
    ic.fill(0, None, 0).expect("Failed to clear display");
    ic
}
