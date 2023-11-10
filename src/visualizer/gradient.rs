#![allow(dead_code)]

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub struct Gradient {
    high_color: Color,
    mid_color: Color,
    low_color: Color,
    max: f32,
    min: f32,
}

impl Gradient {
    pub fn default(max: f32, min: f32) -> Self {
        debug_assert!(max > min);
        let high_color = Color { r: 255, g: 0, b: 0 };
        let mid_color = Color { r: 0, g: 255, b: 0 };
        let low_color = Color { r: 0, g: 0, b: 255 };

        Gradient {
            high_color,
            mid_color,
            low_color,
            max,
            min,
        }
    }
    pub fn get_color(&self, value: f32) -> Color {
        if value > self.max {
            return self.high_color;
        } else if value < self.min {
            return self.low_color;
        } else {
            let mid = (self.max + self.min) / 2.0;
            if value > mid {
                let mul = (value - mid) / (self.max - mid);
                todo!();
                self.high_color
            } else {
                todo!();
                self.high_color
            }
        }
    }
}
