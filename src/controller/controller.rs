extern crate linux_embedded_hal as hal;
extern crate pwm_pca9685 as pca9685;

use crate::led::led::{cie1931, Color, LED};
use hal::I2cdev;
use pca9685::{Address, Channel, Pca9685};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Controller {
    pub pwm: Arc<Mutex<Pca9685<I2cdev>>>,
}

impl Controller {
    pub fn new() -> Self {
        let dev = I2cdev::new("/dev/i2c-1").unwrap();
        let address = Address::default();

        let pwm = Arc::new(Mutex::new(Pca9685::new(dev, address).unwrap()));

        Controller { pwm: pwm }
    }

    pub async fn enable(&mut self) {
        let mut pwm = self.pwm.lock().await;
        pwm.enable().unwrap();
    }
    pub async fn disable(&mut self) {
        let mut pwm = self.pwm.lock().await;
        pwm.disable().unwrap();
    }

    pub async fn apply(&mut self, led: LED) {
        self.set_brightness(Color::White, led.white).await;
        self.set_brightness(Color::Red, led.red).await;
        self.set_brightness(Color::Green, led.green).await;
        self.set_brightness(Color::Blue, led.blue).await;
    }

    async fn set_brightness(&mut self, color: Color, val: f64) {
        let mut pwm = self.pwm.lock().await;

        match color {
            Color::White => {
                pwm.set_channel_on_off(Channel::C0, 0, cie1931(val))
                    .unwrap();
                pwm.set_channel_on_off(Channel::C4, 0, cie1931(val))
                    .unwrap();
                pwm.set_channel_on_off(Channel::C8, 0, cie1931(val))
                    .unwrap();
                pwm.set_channel_on_off(Channel::C12, 0, cie1931(val))
                    .unwrap();
            }
            Color::Red => {
                pwm.set_channel_on_off(Channel::C1, 0, cie1931(val))
                    .unwrap();
                pwm.set_channel_on_off(Channel::C5, 0, cie1931(val))
                    .unwrap();
                pwm.set_channel_on_off(Channel::C9, 0, cie1931(val))
                    .unwrap();
                pwm.set_channel_on_off(Channel::C13, 0, cie1931(val))
                    .unwrap();
            }
            Color::Green => {
                pwm.set_channel_on_off(Channel::C2, 0, cie1931(val))
                    .unwrap();
                pwm.set_channel_on_off(Channel::C6, 0, cie1931(val))
                    .unwrap();
                pwm.set_channel_on_off(Channel::C10, 0, cie1931(val))
                    .unwrap();
                pwm.set_channel_on_off(Channel::C14, 0, cie1931(val))
                    .unwrap();
            }
            Color::Blue => {
                pwm.set_channel_on_off(Channel::C3, 0, cie1931(val))
                    .unwrap();
                pwm.set_channel_on_off(Channel::C7, 0, cie1931(val))
                    .unwrap();
                pwm.set_channel_on_off(Channel::C11, 0, cie1931(val))
                    .unwrap();
                pwm.set_channel_on_off(Channel::C15, 0, cie1931(val))
                    .unwrap();
            }
        }
    }
}
