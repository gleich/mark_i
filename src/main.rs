use rppal::i2c::I2c;

mod clock;

fn main() {
    let i2c = I2c::new().expect("Failed to setup i2c bus");
    let mut ic = clock::setup(i2c);
    clock::run(&mut ic);
}
