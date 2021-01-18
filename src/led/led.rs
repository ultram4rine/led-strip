use serde::{Deserialize, Serialize};

pub enum Color {
    White,
    Red,
    Green,
    Blue,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LED {
    pub white: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl LED {
    pub fn new() -> Self {
        LED {
            white: 0,
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn set_brightness(&mut self, color: Color, val: u8) {
        match color {
            Color::White => self.white = val,
            Color::Red => self.red = val,
            Color::Green => self.green = val,
            Color::Blue => self.blue = val,
        }
    }

    pub fn get_brightness(&mut self, color: Color) -> u8 {
        match color {
            Color::White => self.white,
            Color::Red => self.red,
            Color::Green => self.green,
            Color::Blue => self.blue,
        }
    }
}

pub fn convert8to12(x: u8) -> u16 {
    let a: u32 = x as u32 * ((1 << 12) - 1);
    let b: u32 = (1 << 8) - 1;
    (a / b) as u16
}
