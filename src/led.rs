use serde::{Deserialize, Serialize};

pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct LED {
    pub white: u16,
    pub color: RGB,
}

impl LED {
    pub fn new(white: u16, red: u8, green: u8, blue: u8) -> Self {
        LED {
            white: white,
            color: RGB {
                red: red,
                green: green,
                blue: blue,
            },
        }
    }
}

pub fn convert8to12(x: u8) -> u16 {
    let a: u32 = x as u32 * ((1 << 12) - 1);
    let b: u32 = (1 << 8) - 1;
    (a / b) as u16
}
