pub enum Color {
    White,
    Red,
    Green,
    Blue,
}

pub struct LED {
    pub white: f64,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl LED {
    pub fn new() -> Self {
        LED {
            white: 0.0,
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn set_brightness(&mut self, color: Color, val: f64) {
        match color {
            Color::White => self.white = val,
            Color::Red => self.red = val,
            Color::Green => self.green = val,
            Color::Blue => self.blue = val,
        }
    }

    pub fn get_brightness(&mut self, color: Color) -> f64 {
        match color {
            Color::White => self.white,
            Color::Red => self.red,
            Color::Green => self.green,
            Color::Blue => self.blue,
        }
    }
}

pub fn cie1931(b: f64) -> u16 {
    let led_max = 4095_f64;

    if b <= 8.0 {
        b / 902.3 * led_max;
    }

    let mut x = b + 16_f64 / 116_f64;
    x = x * x * x;

    (x * led_max) as u16
}
