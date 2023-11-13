#![allow(dead_code)]

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
        let mid = (self.max + self.min) / 2.0;

        // the distance how far from mid
        let offset = value - mid;

        // ratio of value
        let ratio = (offset / (mid - self.max)).abs();

        if offset > 0.0 {
            if ratio > 1.0 {
                return self.high_color;
            }

            let high_r = self.high_color.r;
            let high_g = self.high_color.g;
            let high_b = self.high_color.b;

            let mid_r = self.mid_color.r;
            let mid_g = self.mid_color.g;
            let mid_b = self.mid_color.b;

            let r = ((high_r as f32 - mid_r as f32) * ratio + mid_r as f32).round()
                as u8;
            let g = ((high_g as f32 - mid_g as f32) * ratio + mid_g as f32).round()
                as u8;
            let b = ((high_b as f32 - mid_b as f32) * ratio + mid_b as f32).round()
                as u8;
            return Color { r, g, b };
        } else {
            if ratio > 1.0 {
                return self.low_color;
            }

            let low_r = self.low_color.r;
            let low_g = self.low_color.g;
            let low_b = self.low_color.b;

            let mid_r = self.mid_color.r;
            let mid_g = self.mid_color.g;
            let mid_b = self.mid_color.b;

            let r = ((low_r as f32 - mid_r as f32) * ratio + mid_r as f32).round()
                as u8;
            let g = ((low_g as f32 - mid_g as f32) * ratio + mid_g as f32).round()
                as u8;
            let b = ((low_b as f32 - mid_b as f32) * ratio + mid_b as f32).round()
                as u8;
            return Color { r, g, b };
        }
    }
}
