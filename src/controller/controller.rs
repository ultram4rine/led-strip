extern crate linux_embedded_hal as hal;
extern crate pwm_pca9685 as pca9685;

use crate::led::led::{cie1931, LED};
use pca9685::{Address, Channel, Pca9685};

pub struct Controller {
    pub pwm: pca9685::Pca9685<hal::I2cdev>,
}

impl Controller {
    pub fn new() -> Self {
        let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
        let address = Address::default();

        let mut pwm = Pca9685::new(dev, address).unwrap();

        Controller { pwm: pwm }
    }

    pub fn enable(&mut self) {
        self.pwm.enable();
    }
    pub fn disable(&mut self) {
        self.pwm.disable();
    }

    pub fn apply(&mut self, led: LED) {
        self.pwm
            .set_channel_on_off(Channel::C0, 0, cie1931(led.white))
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C4, 0, cie1931(led.white))
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C8, 0, cie1931(led.white))
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C12, 0, cie1931(led.white))
            .unwrap();

        self.pwm
            .set_channel_on_off(Channel::C1, 0, cie1931(led.red))
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C5, 0, cie1931(led.red))
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C9, 0, cie1931(led.red))
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C13, 0, cie1931(led.red))
            .unwrap();

        self.pwm
            .set_channel_on_off(Channel::C2, 0, cie1931(led.green))
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C6, 0, cie1931(led.green))
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C10, 0, cie1931(led.green))
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C14, 0, cie1931(led.green))
            .unwrap();

        self.pwm
            .set_channel_on_off(Channel::C3, 0, cie1931(led.blue))
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C7, 0, cie1931(led.blue))
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C11, 0, cie1931(led.blue))
            .unwrap();
        self.pwm
            .set_channel_on_off(Channel::C15, 0, cie1931(led.blue))
            .unwrap();
    }
}
