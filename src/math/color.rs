use std::f32::*;
use math::*;

#[derive(Debug,PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug,PartialEq)]
pub struct HSL {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}

impl Color {
    pub fn from_floats(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: clamp(r, 0.0, 1.0),
            g: clamp(g, 0.0, 1.0),
            b: clamp(b, 0.0, 1.0),
        }
    }

    pub fn from_ints(r: u32, g: u32, b: u32) -> Color {
        Color::from_floats((r as f32) / 255.0, (g as f32) / 255.0, (b as f32) / 255.0)
    }

    pub fn from_hex(hex: u32) -> Color {
        Color::from_ints(hex >> 16 & 255, hex >> 8 & 255, hex & 255)
    }

    pub fn from_scalar(scalar: f32) -> Color {
        Color::from_floats(scalar, scalar, scalar)
    }

    pub fn from_hsl(hsl: &HSL) -> Color {
        let hue2rgb = |p: f32, q: f32, t: f32| -> f32 {
            let mut mt = t;
            if mt < 0.0 {
                mt += 1.0;
            }
            if mt > 1.0 {
                mt -= 1.0;
            }

            if mt < 1.0 / 6.0 {
                return p + (q - p) * 6.0 * mt;
            }
            
            if mt < 1.0 / 2.0 {
                return q;
            }
            
            if mt < 2.0 / 3.0 {
                return p + (q - p) * 6.0 * (2.0 / 3.0 - mt);
            }
            
            p
        };

        // h,s,l ranges are in 0.0 - 1.0
        let h_clamped = euclidean_modulo(hsl.hue, 1.0);
        let s_clamped = clamp(hsl.saturation, 0.0, 1.0);
        let l = clamp(hsl.lightness, 0.0, 1.0);

        if s_clamped == 0.0 {
            Color::from_scalar(1.0)
        } else {
            let p = if l <= 0.5 {
                l * (1.0 + s_clamped)
            } else {
                l + s_clamped - (l * s_clamped)
            };
            let q = (2.0 * l) - p;
            let one_third = 1.0 / 3.0;
            Color {
                r: hue2rgb(q, p, h_clamped + one_third),
                g: hue2rgb(q, p, h_clamped),
                b: hue2rgb(q, p, h_clamped - one_third),
            }
        }
    }


    pub fn gamma_to_linear(&self, gamma_factor: Option<f32>) -> Color {
        let g = match gamma_factor {
            Some(x) => x,
            None => 2.0,
        };

        Color {
            r: self.r.powf(g),
            g: self.g.powf(g),
            b: self.b.powf(g),
        }
    }

    pub fn linear_to_gamma(&self, gamma_factor: Option<f32>) -> Color {
        let g = match gamma_factor {
            Some(x) => x,
            None => 2.0,
        };

        let safe_inverse = if g > 0.0 { 1.0 / g } else { 1.0 };

        Color {
            r: self.r.powf(safe_inverse),
            g: self.g.powf(safe_inverse),
            b: self.b.powf(safe_inverse),
        }
    }

    pub fn convert_gamma_to_linear(&self) -> Color {
        Color {
            r: self.r * self.r,
            g: self.g * self.g,
            b: self.b * self.b,
        }
    }

    pub fn convert_linear_to_gamma(&self) -> Color {
        Color {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }

    pub fn hex(&self) -> u32 {
        let r = (self.r * 255.0) as u32;
        let g = (self.g * 255.0) as u32;
        let b = (self.b * 255.0) as u32;
        r << 16 ^ g << 8 ^ b
    }

    pub fn hex_string(&self) -> String {
        format!("{:x}", self.hex())
    }

    pub fn hsl(&self) -> HSL {
        // h,s,l ranges are in 0.0 - 1.0
        let &Color { r, g, b } = self;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);

        let lightness = (min + max) / 2.0;

        if (min - max).abs() < EPSILON {
            HSL {
                hue: 0.0,
                saturation: 0.0,
                lightness: lightness,
            }
        } else {
            let delta = max - min;
            let saturation = if lightness <= 0.5 {
                delta / (max + min)
            } else {
                delta / (2.0 - max - min)
            };

            let hue = match max {
                _ if (max - r).abs() < EPSILON => (g - b) / delta + (if g < b { 6.0 } else { 0.0 }),
                _ if (max - g).abs() < EPSILON => (b - r) / delta + 2.0,
                _ if (max - b).abs() < EPSILON => (r - g) / delta + 4.0,
                _ => panic!("Oh noes"),
            };

            HSL {
                hue: hue / 6.0,
                saturation: saturation,
                lightness: lightness,
            }
        }
    }

    pub fn offset_hsl(&self, hsl: &HSL) -> Color {
        let mut _hsl = self.hsl();
        _hsl.hue += hsl.hue;
        _hsl.saturation += hsl.saturation;
        _hsl.lightness += hsl.lightness;
        Color::from_hsl(&_hsl)
    }

    pub fn add(&self, color: &Color) -> Color {
        Color::from_floats(self.r + color.r, self.g + color.g, self.b + color.b)
    }

    pub fn add_scalar(&self, s: f32) -> Color {
        Color::from_floats(self.r + s, self.g + s, self.b + s)
    }

    pub fn subtract(&self, color: &Color) -> Color {
        Color::from_floats(self.r - color.r, self.g - color.g, self.b - color.b)
    }

    pub fn multiply(&self, color: &Color) -> Color {
        Color::from_floats(self.r * color.r, self.g * color.g, self.b * color.b)
    }

    pub fn multiply_scalar(&self, s: f32) -> Color {
        Color::from_floats(self.r * s, self.g * s, self.b * s)
    }

    pub fn lerp(&self, color: &Color, alpha: f32) -> Color {
        Color {
            r: self.r + ((color.r - self.r) * alpha),
            g: self.g + ((color.g - self.g) * alpha),
            b: self.b + ((color.b - self.b) * alpha),
        }
    }
}

pub const MAROON: Color = Color {
    r: 0.50196,
    g: 0.0,
    b: 0.0,
};

pub const DARK_RED: Color = Color {
    r: 0.54510,
    g: 0.0,
    b: 0.0,
};

pub const BROWN: Color = Color {
    r: 0.64706,
    g: 0.16471,
    b: 0.16471,
};

pub const FIREBRICK: Color = Color {
    r: 0.69804,
    g: 0.13333,
    b: 0.13333,
};

pub const CRIMSON: Color = Color {
    r: 0.86275,
    g: 0.07843,
    b: 0.23529,
};

pub const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
};

pub const TOMATO: Color = Color {
    r: 1.0,
    g: 0.38824,
    b: 0.27843,
};

pub const CORAL: Color = Color {
    r: 1.0,
    g: 0.49804,
    b: 80.0 / 255.0,
};

pub const INDIAN_RED: Color = Color {
    r: 205.0 / 255.0,
    g: 92.0 / 255.0,
    b: 92.0 / 255.0,
};

pub const LIGHT_CORAL: Color = Color {
    r: 240.0 / 255.0,
    g: 0.50196,
    b: 0.50196,
};

pub const DARK_SALMON: Color = Color {
    r: 233.0 / 255.0,
    g: 150.0 / 255.0,
    b: 122.0 / 255.0,
};

pub const SALMON: Color = Color {
    r: 250.0 / 255.0,
    g: 0.50196,
    b: 114.0 / 255.0,
};

pub const LIGHT_SALMON: Color = Color {
    r: 1.0,
    g: 160.0 / 255.0,
    b: 122.0 / 255.0,
};

pub const ORANGE_RED: Color = Color {
    r: 1.0,
    g: 69.0 / 255.0,
    b: 0.0,
};

pub const DARK_ORANGE: Color = Color {
    r: 1.0,
    g: 140.0 / 255.0,
    b: 0.0,
};

pub const ORANGE: Color = Color {
    r: 1.0,
    g: 0.64706,
    b: 0.0,
};

pub const GOLD: Color = Color {
    r: 1.0,
    g: 215.0 / 255.0,
    b: 0.0,
};

pub const DARK_GOLDEN_ROD: Color = Color {
    r: 184.0 / 255.0,
    g: 134.0 / 255.0,
    b: 11.0 / 255.0,
};

pub const GOLDEN_ROD: Color = Color {
    r: 218.0 / 255.0,
    g: 0.64706,
    b: 32.0 / 255.0,
};

pub const PALE_GOLDEN_ROD: Color = Color {
    r: 238.0 / 255.0,
    g: 232.0 / 255.0,
    b: 170.0 / 255.0,
};

pub const DARK_KHAKI: Color = Color {
    r: 189.0 / 255.0,
    g: 183.0 / 255.0,
    b: 107.0 / 255.0,
};

pub const KHAKI: Color = Color {
    r: 240.0 / 255.0,
    g: 230.0 / 255.0,
    b: 140.0 / 255.0,
};

pub const OLIVE: Color = Color {
    r: 0.50196,
    g: 0.50196,
    b: 0.0,
};

pub const YELLOW: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 0.0,
};

pub const YELLOW_GREEN: Color = Color {
    r: 154.0 / 255.0,
    g: 205.0 / 255.0,
    b: 50.0 / 255.0,
};

pub const DARK_OLIVE_GREEN: Color = Color {
    r: 85.0 / 255.0,
    g: 107.0 / 255.0,
    b: 47.0 / 255.0,
};

pub const OLIVE_DRAB: Color = Color {
    r: 107.0 / 255.0,
    g: 142.0 / 255.0,
    b: 35.0 / 255.0,
};

pub const LAWN_GREEN: Color = Color {
    r: 124.0 / 255.0,
    g: 252.0 / 255.0,
    b: 0.0,
};

pub const CHARTREUSE: Color = Color {
    r: 0.49804,
    g: 1.0,
    b: 0.0,
};

pub const GREEN_YELLOW: Color = Color {
    r: 173.0 / 255.0,
    g: 1.0,
    b: 47.0 / 255.0,
};

pub const DARK_GREEN: Color = Color {
    r: 0.0,
    g: 100.0 / 255.0,
    b: 0.0,
};

pub const GREEN: Color = Color {
    r: 0.0,
    g: 0.50196,
    b: 0.0,
};

pub const FOREST_GREEN: Color = Color {
    r: 0.13333,
    g: 0.54510,
    b: 0.13333,
};

pub const LIME: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
};

pub const LIME_GREEN: Color = Color {
    r: 50.0 / 255.0,
    g: 205.0 / 255.0,
    b: 50.0 / 255.0,
};

pub const LIGHT_GREEN: Color = Color {
    r: 144.0 / 255.0,
    g: 238.0 / 255.0,
    b: 144.0 / 255.0,
};

pub const PALE_GREEN: Color = Color {
    r: 0.59608,
    g: 251.0 / 255.0,
    b: 0.59608,
};

pub const DARK_SEA_GREEN: Color = Color {
    r: 143.0 / 255.0,
    g: 188.0 / 255.0,
    b: 143.0 / 255.0,
};

pub const MEDIUM_SPRING_GREEN: Color = Color {
    r: 0.0,
    g: 250.0 / 255.0,
    b: 154.0 / 255.0,
};

pub const SPRING_GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.49804,
};

pub const SEA_GREEN: Color = Color {
    r: 46.0 / 255.0,
    g: 0.54510,
    b: 87.0 / 255.0,
};

pub const MEDIUM_AQUA_MARINE: Color = Color {
    r: 102.0 / 255.0,
    g: 205.0 / 255.0,
    b: 170.0 / 255.0,
};

pub const MEDIUM_SEA_GREEN: Color = Color {
    r: 0.23529,
    g: 179.0 / 255.0,
    b: 113.0 / 255.0,
};

pub const LIGHT_SEA_GREEN: Color = Color {
    r: 32.0 / 255.0,
    g: 0.69804,
    b: 170.0 / 255.0,
};

pub const DARK_SLATE_GRAY: Color = Color {
    r: 47.0 / 255.0,
    g: 79.0 / 255.0,
    b: 79.0 / 255.0,
};

pub const TEAL: Color = Color {
    r: 0.0,
    g: 0.50196,
    b: 0.50196,
};

pub const DARK_CYAN: Color = Color {
    r: 0.0,
    g: 0.54510,
    b: 0.54510,
};

pub const AQUA: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 1.0,
};

pub const CYAN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 1.0,
};

pub const LIGHT: Color = Color {
    r: 224.0 / 255.0,
    g: 1.0,
    b: 1.0,
};

pub const DARK_TURQUOISE: Color = Color {
    r: 0.0,
    g: 206.0 / 255.0,
    b: 209.0 / 255.0,
};

pub const TURQUOISE: Color = Color {
    r: 64.0 / 255.0,
    g: 224.0 / 255.0,
    b: 208.0 / 255.0,
};

pub const MEDIUM_TURQUOISE: Color = Color {
    r: 72.0 / 255.0,
    g: 209.0 / 255.0,
    b: 204.0 / 255.0,
};

pub const PALE_TURQUOISE: Color = Color {
    r: 175.0 / 255.0,
    g: 238.0 / 255.0,
    b: 238.0 / 255.0,
};

pub const AQUA_MARINE: Color = Color {
    r: 0.49804,
    g: 1.0,
    b: 212.0 / 255.0,
};

pub const POWDER_BLUE: Color = Color {
    r: 176.0 / 255.0,
    g: 224.0 / 255.0,
    b: 230.0 / 255.0,
};

pub const CADET_BLUE: Color = Color {
    r: 95.0 / 255.0,
    g: 158.0 / 255.0,
    b: 160.0 / 255.0,
};

pub const STEEL_BLUE: Color = Color {
    r: 70.0 / 255.0,
    g: 130.0 / 255.0,
    b: 180.0 / 255.0,
};

pub const CORNFLOWER_BLUE: Color = Color {
    r: 100.0 / 255.0,
    g: 149.0 / 255.0,
    b: 237.0 / 255.0,
};

pub const DEEP_SKY_BLUE: Color = Color {
    r: 0.0,
    g: 191.0 / 255.0,
    b: 1.0,
};

pub const DODGER_BLUE: Color = Color {
    r: 30.0 / 255.0,
    g: 144.0 / 255.0,
    b: 55.0 / 255.0,
};

pub const LIGHT_BLUE: Color = Color {
    r: 173.0 / 255.0,
    g: 216.0 / 255.0,
    b: 230.0 / 255.0,
};

pub const SKY_BLUE: Color = Color {
    r: 135.0 / 255.0,
    g: 206.0 / 255.0,
    b: 235.0 / 255.0,
};

pub const LIGHT_SKY_BLUE: Color = Color {
    r: 135.0 / 255.0,
    g: 206.0 / 255.0,
    b: 250.0 / 255.0,
};

pub const MIDNIGHT_BLUE: Color = Color {
    r: 25.0 / 255.0,
    g: 25.0 / 255.0,
    b: 112.0 / 255.0,
};

pub const NAVY: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.50196,
};

pub const DARK_BLUE: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.54510,
};

pub const MEDIUM_BLUE: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 205.0 / 255.0,
};

pub const BLUE: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 1.0,
};

pub const ROYAL_BLUE: Color = Color {
    r: 65.0 / 255.0,
    g: 105.0 / 255.0,
    b: 225.0 / 255.0,
};

pub const BLUE_VIOLET: Color = Color {
    r: 138.0 / 255.0,
    g: 43.0 / 255.0,
    b: 226.0 / 255.0,
};

pub const INDIGO: Color = Color {
    r: 75.0 / 255.0,
    g: 0.0,
    b: 130.0 / 255.0,
};

pub const DARK_SLATE_BLUE: Color = Color {
    r: 72.0 / 255.0,
    g: 61.0 / 255.0,
    b: 0.54510,
};

pub const SLATE_BLUE: Color = Color {
    r: 106.0 / 255.0,
    g: 90.0 / 255.0,
    b: 205.0 / 255.0,
};

pub const MEDIUM_SLATE_BLUE: Color = Color {
    r: 123.0 / 255.0,
    g: 104.0 / 255.0,
    b: 238.0 / 255.0,
};

pub const MEDIUM_PURPLE: Color = Color {
    r: 147.0 / 255.0,
    g: 112.0 / 255.0,
    b: 219.0 / 255.0,
};

pub const DARK_MAGENTA: Color = Color {
    r: 0.54510,
    g: 0.0,
    b: 0.54510,
};

pub const DARK_VIOLET: Color = Color {
    r: 148.0 / 255.0,
    g: 0.0,
    b: 211.0 / 255.0,
};

pub const DARK_ORCHID: Color = Color {
    r: 153.0 / 255.0,
    g: 50.0 / 255.0,
    b: 204.0 / 255.0,
};

pub const MEDIUM_ORCHID: Color = Color {
    r: 186.0 / 255.0,
    g: 85.0 / 255.0,
    b: 211.0 / 255.0,
};

pub const PURPLE: Color = Color {
    r: 0.50196,
    g: 0.0,
    b: 0.50196,
};

pub const THISTLE: Color = Color {
    r: 216.0 / 255.0,
    g: 191.0 / 255.0,
    b: 216.0 / 255.0,
};

pub const PLUM: Color = Color {
    r: 221.0 / 255.0,
    g: 160.0 / 255.0,
    b: 221.0 / 255.0,
};

pub const VIOLET: Color = Color {
    r: 238.0 / 255.0,
    g: 130.0 / 255.0,
    b: 238.0 / 255.0,
};

pub const MAGENTA: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 1.0,
};

pub const ORCHID: Color = Color {
    r: 218.0 / 255.0,
    g: 112.0 / 255.0,
    b: 214.0 / 255.0,
};

pub const MEDIUM_VIOLET_RED: Color = Color {
    r: 199.0 / 255.0,
    g: 21.0 / 255.0,
    b: 133.0 / 255.0,
};

pub const PALE_VIOLET_RED: Color = Color {
    r: 219.0 / 255.0,
    g: 112.0 / 255.0,
    b: 147.0 / 255.0,
};

pub const DEEP_PINK: Color = Color {
    r: 1.0,
    g: 0.07843,
    b: 47.0 / 255.0,
};

pub const HOT_PINK: Color = Color {
    r: 1.0,
    g: 105.0 / 255.0,
    b: 180.0 / 255.0,
};

pub const LIGHT_PINK: Color = Color {
    r: 1.0,
    g: 182.0 / 255.0,
    b: 193.0 / 255.0,
};

pub const PINK: Color = Color {
    r: 1.0,
    g: 192.0 / 255.0,
    b: 203.0 / 255.0,
};

pub const ANTIQUE_WHITE: Color = Color {
    r: 250.0 / 255.0,
    g: 235.0 / 255.0,
    b: 215.0 / 255.0,
};

pub const BEIGE: Color = Color {
    r: 245.0 / 255.0,
    g: 245.0 / 255.0,
    b: 0.86275,
};

pub const BISQUE: Color = Color {
    r: 1.0,
    g: 228.0 / 255.0,
    b: 96.0 / 255.0,
};

pub const BLANCHED_ALMOND: Color = Color {
    r: 1.0,
    g: 235.0 / 255.0,
    b: 205.0 / 255.0,
};

pub const WHEAT: Color = Color {
    r: 245.0 / 255.0,
    g: 222.0 / 255.0,
    b: 179.0 / 255.0,
};

pub const CORN_SILK: Color = Color {
    r: 1.0,
    g: 248.0 / 255.0,
    b: 0.86275,
};

pub const LEMON_CHIFFON: Color = Color {
    r: 1.0,
    g: 250.0 / 255.0,
    b: 205.0 / 255.0,
};

pub const LIGHT_GOLDENROD_YELLOW: Color = Color {
    r: 250.0 / 255.0,
    g: 250.0 / 255.0,
    b: 210.0 / 255.0,
};

pub const LIGHT_YELLOW: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 224.0 / 255.0,
};

pub const SADDLE_BROWN: Color = Color {
    r: 0.54510,
    g: 69.0 / 255.0,
    b: 19.0 / 255.0,
};

pub const SIENNA: Color = Color {
    r: 160.0 / 255.0,
    g: 82.0 / 255.0,
    b: 45.0 / 255.0,
};

pub const CHOCOLATE: Color = Color {
    r: 210.0 / 255.0,
    g: 105.0 / 255.0,
    b: 30.0 / 255.0,
};

pub const PERU: Color = Color {
    r: 205.0 / 255.0,
    g: 133.0 / 255.0,
    b: 63.0 / 255.0,
};

pub const SANDY_BROWN: Color = Color {
    r: 244.0 / 255.0,
    g: 164.0 / 255.0,
    b: 96.0 / 255.0,
};

pub const BURLY_WOOD: Color = Color {
    r: 222.0 / 255.0,
    g: 184.0 / 255.0,
    b: 135.0 / 255.0,
};

pub const TAN: Color = Color {
    r: 210.0 / 255.0,
    g: 180.0 / 255.0,
    b: 140.0 / 255.0,
};

pub const ROSY_BROWN: Color = Color {
    r: 188.0 / 255.0,
    g: 143.0 / 255.0,
    b: 143.0 / 255.0,
};

pub const MOCCASIN: Color = Color {
    r: 1.0,
    g: 228.0 / 255.0,
    b: 181.0 / 255.0,
};

pub const NAVAJO_WHITE: Color = Color {
    r: 1.0,
    g: 222.0 / 255.0,
    b: 173.0 / 255.0,
};

pub const PEACH_PUFF: Color = Color {
    r: 1.0,
    g: 218.0 / 255.0,
    b: 185.0 / 255.0,
};

pub const MISTY_ROSE: Color = Color {
    r: 1.0,
    g: 228.0 / 255.0,
    b: 225.0 / 255.0,
};

pub const LAVENDER_BLUSH: Color = Color {
    r: 1.0,
    g: 240.0 / 255.0,
    b: 245.0 / 255.0,
};

pub const LINEN: Color = Color {
    r: 250.0 / 255.0,
    g: 240.0 / 255.0,
    b: 230.0 / 255.0,
};

pub const OLD_LACE: Color = Color {
    r: 253.0 / 255.0,
    g: 245.0 / 255.0,
    b: 230.0 / 255.0,
};

pub const PAPAYA_WHIP: Color = Color {
    r: 1.0,
    g: 239.0 / 255.0,
    b: 213.0 / 255.0,
};

pub const SEA_SHELL: Color = Color {
    r: 1.0,
    g: 245.0 / 255.0,
    b: 238.0 / 255.0,
};

pub const MINT_CREAM: Color = Color {
    r: 245.0 / 255.0,
    g: 1.0,
    b: 250.0 / 255.0,
};

pub const SLATE_GRAY: Color = Color {
    r: 112.0 / 255.0,
    g: 0.50196,
    b: 144.0 / 255.0,
};

pub const LIGHT_SLATE_GRAY: Color = Color {
    r: 119.0 / 255.0,
    g: 136.0 / 255.0,
    b: 153.0 / 255.0,
};

pub const LIGHT_STEEL_BLUE: Color = Color {
    r: 176.0 / 255.0,
    g: 196.0 / 255.0,
    b: 222.0 / 255.0,
};

pub const LAVENDER: Color = Color {
    r: 230.0 / 255.0,
    g: 230.0 / 255.0,
    b: 250.0 / 255.0,
};

pub const FLORAL_WHITE: Color = Color {
    r: 1.0,
    g: 250.0 / 255.0,
    b: 240.0 / 255.0,
};

pub const ALICE_BLUE: Color = Color {
    r: 240.0 / 255.0,
    g: 248.0 / 255.0,
    b: 1.0,
};

pub const GHOST_WHITE: Color = Color {
    r: 248.0 / 255.0,
    g: 248.0 / 255.0,
    b: 1.0,
};

pub const HONEYDEW: Color = Color {
    r: 240.0 / 255.0,
    g: 1.0,
    b: 240.0 / 255.0,
};

pub const IVORY: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 240.0 / 255.0,
};

pub const AZURE: Color = Color {
    r: 240.0 / 255.0,
    g: 1.0,
    b: 1.0,
};

pub const SNOW: Color = Color {
    r: 1.0,
    g: 250.0 / 255.0,
    b: 250.0 / 255.0,
};

pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};

pub const DIM_GRAY_DIM_GREY: Color = Color {
    r: 105.0 / 255.0,
    g: 105.0 / 255.0,
    b: 105.0 / 255.0,
};

pub const GRAY_GREY: Color = Color {
    r: 0.50196,
    g: 0.50196,
    b: 0.50196,
};

pub const DARK_GRAY_DARK_GREY: Color = Color {
    r: 169.0 / 255.0,
    g: 169.0 / 255.0,
    b: 169.0 / 255.0,
};

pub const SILVER: Color = Color {
    r: 192.0 / 255.0,
    g: 192.0 / 255.0,
    b: 192.0 / 255.0,
};

pub const LIGHT_GRAY_LIGHT_GREY: Color = Color {
    r: 211.0 / 255.0,
    g: 211.0 / 255.0,
    b: 211.0 / 255.0,
};

pub const GAINSBORO: Color = Color {
    r: 0.86275,
    g: 0.86275,
    b: 0.86275,
};

pub const WHITE_SMOKE: Color = Color {
    r: 245.0 / 255.0,
    g: 245.0 / 255.0,
    b: 245.0 / 255.0,
};

pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};

#[cfg(test)]
mod tests {
    use math::*;

    #[test]
    fn set_rgb() {
        let c = Color::from_floats(1.0, 0.2, 0.1);
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.2);
        assert_eq!(c.b, 0.1);
    }

    #[test]
    fn copy_gamma_to_linear() {
        let c2 = Color::from_floats(0.3, 0.5, 0.9);
        let c = c2.gamma_to_linear(None);
        assert_eq!(c.r, 0.09);
        assert_eq!(c.g, 0.25);
        assert_eq!(c.b, 0.80999994);
    }

    #[test]
    fn copy_linear_to_gamma() {
        let c2 = Color::from_floats(0.09, 0.25, 0.81);
        let c = c2.linear_to_gamma(None);
        assert_eq!(c.r, 0.3);
        assert_eq!(c.g, 0.5);
        assert_eq!(c.b, 0.9);
    }


    #[test]
    fn convert_gamma_to_linear() {
        let c = Color::from_floats(0.3, 0.5, 0.9).convert_gamma_to_linear();
        assert_eq!(c.r, 0.09);
        assert_eq!(c.g, 0.25);
        assert_eq!(c.b, 0.80999994);
    }


    #[test]
    fn convert_linear_to_gamma() {
        let c = Color {
                r: 4.0,
                g: 9.0,
                b: 16.0,
            }
            .convert_linear_to_gamma();
        assert_eq!(c.r, 2.0);
        assert_eq!(c.g, 3.0);
        assert_eq!(c.b, 4.0);
    }

    #[test]
    fn set_with_num() {
        let c = Color::from_hex(0xFF0000);
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.0);
    }


    #[test]
    fn lerp() {
        let c = Color::from_ints(0, 0, 0);
        let c2 = c.lerp(&WHITE, 0.2);
        assert_eq!(c2.r, 0.2);
        assert_eq!(c2.g, 0.2);
        assert_eq!(c2.b, 0.2);
    }

    #[test]
    fn get_hex() {
        let res = RED.hex();
        assert_eq!(res, 0xFF0000);
    }

    #[test]
    fn set_hex() {
        let c = Color::from_hex(0xFA8072);
        assert_eq!(c.hex(), 0xFA8072);
    }

    #[test]
    fn get_hex_string() {
        let res = TOMATO.hex_string();
        assert_eq!(res, "ff6346");
    }

    #[test]
    fn get_hsl() {
        let c = Color::from_hex(0x80ffff);
        let hsl = c.hsl();

        assert_eq!(hsl.hue, 0.5);
        assert_eq!(hsl.saturation, 1.0);
        assert_eq!((hsl.lightness * 100.0).round() / 100.0, 0.75);
    }

    #[test]
    fn set_hsl() {
        let c = Color::from_hsl(&HSL {
            hue: 0.75,
            saturation: 1.0,
            lightness: 0.25,
        });
        let hsl = c.hsl();

        assert_eq!(hsl.hue, 0.75);
        assert_eq!(hsl.saturation, 1.00);
        assert_eq!(hsl.lightness, 0.25);
    }
}