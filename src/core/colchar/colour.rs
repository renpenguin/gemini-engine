use std::{
    ops::{Add, AddAssign, Mul, MulAssign},
    str::FromStr,
};

/// Only used on f64 values between 0.0 and 255.0
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn mul_by_f64_to_u8<T: Into<f64>>(value: T, rhs: f64) -> u8 {
    (value.into() * rhs).round() as u8
}

/// A struct to store colour values. Can be created from RGB, HSV or greyscale values, but is ultimately stored as RGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colour {
    /// The red channel of the colour
    pub r: u8,
    /// The green channel of the colour
    pub g: u8,
    /// The blue channel of the colour
    pub b: u8,
}

impl Colour {
    /// A white `Colour` of RGB (0,0,0)
    pub const BLACK: Self = Self::greyscale(0);
    /// A white `Colour` of RGB (255,255,255)
    pub const WHITE: Self = Self::greyscale(255);

    /// Create a `Colour` from an RGB value
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create a `Colour` from an HSV value
    #[must_use]
    pub fn hsv(hue: u8, sat: u8, val: u8) -> Self {
        let hue = f32::from(hue) / 255.0;
        let sat = f32::from(sat) / 255.0;
        let val = f32::from(val) / 255.0;

        let index = (hue * 6.0).floor();
        let f = hue.mul_add(6.0, -index);
        let p = val * f.mul_add(-sat, 1.0);
        let q = val * f.mul_add(-sat, 1.0);
        let t = val * (1.0 - f).mul_add(-sat, 1.0);

        let (red, green, blue) = [
            (val, t, p),
            (q, val, p),
            (p, val, t),
            (p, q, val),
            (t, p, val),
            (val, p, q),
        ][index as usize];

        Self::rgb(
            mul_by_f64_to_u8(red, 255.0),
            mul_by_f64_to_u8(green, 255.0),
            mul_by_f64_to_u8(blue, 255.0),
        )
    }

    /// Create a `Colour` from a single brightness value, resulting in a shade of grey
    #[must_use]
    pub const fn greyscale(v: u8) -> Self {
        Self::rgb(v, v, v)
    }
}

impl Add for Colour {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::rgb(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul<f64> for Colour {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::rgb(
            mul_by_f64_to_u8(self.r, rhs),
            mul_by_f64_to_u8(self.g, rhs),
            mul_by_f64_to_u8(self.b, rhs),
        )
    }
}

impl MulAssign<f64> for Colour {
    fn mul_assign(&mut self, rhs: f64) {
        self.r = mul_by_f64_to_u8(self.r, rhs);
        self.r = mul_by_f64_to_u8(self.g, rhs);
        self.r = mul_by_f64_to_u8(self.b, rhs);
    }
}

impl FromStr for Colour {
    type Err = String;

    /// Colours should be passed in the format `<r>,<g>,<b>`, for example `255,0,0` for red
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(' ', "");
        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() != 3 {
            return Err(String::from(
                "Incorrect number of arguments, string must be in format r,g,b to be parsed correctly",
            ));
        }
        println!("{parts:?}");

        let mut nums = [0u8; 3];

        for i in 0..3 {
            nums[i] = parts[i].parse::<u8>().map_err(|_| {
                String::from("Could not parse part of argument, make sure it's a valid number")
            })?;
        }

        Ok(Self::rgb(nums[0], nums[1], nums[2]))
    }
}
