extern crate linux_embedded_hal as hal;
extern crate pwm_pca9685 as pca9685;

use crate::led::led::{convert8to12, Color, LED};
use hal::I2cdev;
use pca9685::{Address, Channel, Pca9685};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

#[derive(Clone, Serialize)]
pub struct Controller {
    #[serde(skip_serializing)]
    pub pwm: Arc<Mutex<Pca9685<I2cdev>>>,
    pub on: bool,
    pub led: LED,
}

impl Controller {
    pub fn new() -> Self {
        let dev = I2cdev::new("/dev/i2c-1").unwrap();
        let address = Address::default();

        let pwm = Pca9685::new(dev, address).unwrap();

        Controller {
            pwm: Arc::new(Mutex::new(pwm)),
            on: false,
            led: LED {
                white: 0,
                red: 0,
                green: 0,
                blue: 0,
            },
        }
    }

    pub async fn enable(
        &mut self,
    ) -> Result<(), pca9685::Error<hal::i2cdev::linux::LinuxI2CError>> {
        let mut pwm = self.pwm.lock().await;
        pwm.enable()?;
        self.on = true;
        Ok(())
    }
    pub async fn disable(
        &mut self,
    ) -> Result<(), pca9685::Error<hal::i2cdev::linux::LinuxI2CError>> {
        let mut pwm = self.pwm.lock().await;
        pwm.disable()?;
        self.on = false;
        Ok(())
    }

    pub async fn apply(
        &mut self,
        led: LED,
    ) -> Result<(), pca9685::Error<hal::i2cdev::linux::LinuxI2CError>> {
        self.led = led;
        self.set_brightness(Color::White, self.led.white).await?;
        self.set_brightness(Color::Red, self.led.red).await?;
        self.set_brightness(Color::Green, self.led.green).await?;
        self.set_brightness(Color::Blue, self.led.blue).await?;

        Ok(())
    }

    pub async fn twinkle(
        &mut self,
        led: LED,
    ) -> Result<(), pca9685::Error<hal::i2cdev::linux::LinuxI2CError>> {
        let current = self.led;

        for _ in 1..=10 {
            self.apply(LED::new(0, 0, 0, 0)).await?;
            sleep(Duration::from_millis(300)).await;
            self.apply(led).await?;
            sleep(Duration::from_millis(300)).await;
        }

        self.apply(current).await?;

        Ok(())
    }

    async fn set_brightness(
        &mut self,
        color: Color,
        val: u8,
    ) -> Result<(), pca9685::Error<hal::i2cdev::linux::LinuxI2CError>> {
        let mut pwm = self.pwm.lock().await;

        match color {
            Color::White => {
                pwm.set_channel_on_off(Channel::C0, 0, convert8to12(val))?;
                pwm.set_channel_on_off(Channel::C4, 0, convert8to12(val))?;
                pwm.set_channel_on_off(Channel::C8, 0, convert8to12(val))?;
                pwm.set_channel_on_off(Channel::C12, 0, convert8to12(val))?;
            }
            Color::Red => {
                pwm.set_channel_on_off(Channel::C1, 0, convert8to12(val))?;
                pwm.set_channel_on_off(Channel::C5, 0, convert8to12(val))?;
                pwm.set_channel_on_off(Channel::C9, 0, convert8to12(val))?;
                pwm.set_channel_on_off(Channel::C13, 0, convert8to12(val))?;
            }
            Color::Green => {
                pwm.set_channel_on_off(Channel::C2, 0, convert8to12(val))?;
                pwm.set_channel_on_off(Channel::C6, 0, convert8to12(val))?;
                pwm.set_channel_on_off(Channel::C10, 0, convert8to12(val))?;
                pwm.set_channel_on_off(Channel::C14, 0, convert8to12(val))?;
            }
            Color::Blue => {
                pwm.set_channel_on_off(Channel::C3, 0, convert8to12(val))?;
                pwm.set_channel_on_off(Channel::C7, 0, convert8to12(val))?;
                pwm.set_channel_on_off(Channel::C11, 0, convert8to12(val))?;
                pwm.set_channel_on_off(Channel::C15, 0, convert8to12(val))?;
            }
        }

        Ok(())
    }
}
