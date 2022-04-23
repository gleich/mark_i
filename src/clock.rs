use std::{thread, time::Duration};

use chrono::{Datelike, Local, Timelike};
use is31fl3731::{devices::CharlieBonnet, IS31FL3731};
use rppal::{
    hal,
    i2c::{self, I2c},
};

pub fn run() {
    let mut ic = setup(I2c::with_bus(3).expect("Failed to setup i2c bus"));

    let mut old_ms_time = (1, 1);
    let mut old_s_time = (1, 1);
    let mut old_m_time = (1, 1);
    let mut old_h_time = (1, 1);
    let mut old_d_time = (1, 1);
    let mut old_w_time = (1, 1);

    loop {
        let now = Local::now();
        old_ms_time = set_pixel(
            &mut ic,
            coordinate(
                (now.timestamp_subsec_micros() % 100000) as f32 / 100_000.0,
                0,
            ),
            old_ms_time,
        );
        old_s_time = set_pixel(
            &mut ic,
            coordinate(now.timestamp_subsec_millis() as f32 / 1_000.0, 1),
            old_s_time,
        );
        old_m_time = set_pixel(
            &mut ic,
            coordinate(now.second() as f32 / 59.0, 2),
            old_m_time,
        );
        old_h_time = set_pixel(
            &mut ic,
            coordinate(now.minute() as f32 / 59.0, 3),
            old_h_time,
        );
        old_d_time = set_pixel(&mut ic, coordinate(now.hour() as f32 / 23.0, 4), old_d_time);
        old_w_time = set_pixel(
            &mut ic,
            coordinate((now.weekday() as usize) as f32 / 6.0, 5),
            old_w_time,
        );
    }
}

fn coordinate(percent: f32, level: u8) -> (u8, u8) {
    let mut x = level * 3;
    let y;
    let mut loc = (16.0 * percent).round() as u8;
    if loc == 16 {
        loc -= 1;
    }
    if loc > 7 {
        x += 1;
        y = 7 - (loc - 8);
    } else {
        y = loc;
    }
    (x, y)
}

fn set_pixel(ic: &mut IS31FL3731<i2c::I2c>, loc: (u8, u8), old_loc: (u8, u8)) -> (u8, u8) {
    if loc == old_loc {
        return old_loc;
    }
    ic.pixel(old_loc.0, old_loc.1, 1)
        .expect("Failed to set old pixel location to low");
    let mut brightness = 30;
    if loc.1 == 0 {
        brightness = 50;
    }
    ic.pixel(loc.0, loc.1, brightness)
        .expect("Failed to set new pixel location to high");
    loc
}

fn setup(i2c: I2c) -> IS31FL3731<i2c::I2c> {
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
