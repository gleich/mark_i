use std::thread;

use rppal::i2c::I2c;
use unicorn_hat_hd_2::UnicornHatHd;

mod clock;

fn main() {
    // thread::spawn(move || {
    //     let i2c = I2c::new().expect("Failed to setup i2c bus");
    //     let mut ic = clock::setup(i2c);
    //     clock::run(&mut ic);
    // });
    thread::spawn(move || {
        let mut hat_hd = UnicornHatHd::default();
        hat_hd.clear_pixels();
        hat_hd.set_pixel(0, 0, [255, 255, 255].into());
        hat_hd.set_pixel(1, 0, [255, 0, 0].into());
        hat_hd.set_pixel(2, 0, [0, 255, 0].into());
        hat_hd.set_pixel(3, 0, [0, 0, 255].into());
        hat_hd.display().unwrap();
    });
    loop {}
}
