//! Contains the HSV (hue, saturation, value) representation of a color.

#[cfg(feature="no-std")]
use core::mem::transmute;
#[cfg(not(feature="no-std"))]
use std::mem::transmute;

#[cfg(feature="no-std")]
use core::hint::unreachable_unchecked;
#[cfg(not(feature="no-std"))]
use std::hint::unreachable_unchecked;

#[cfg(feature="no-std")]
use core::fmt;
#[cfg(not(feature="no-std"))]
use std::fmt;

use crate::ColorRGB;
use crate::scale::*;

const HSV_SECTION_3: u8 = 0x40;

/// Converts hue to RGB at full brightness and full saturation.
#[inline]
pub fn hue_to_full_rgb(hue: u8) -> ColorRGB {
    let offset: u8 = hue & 0x1F;
    let offset8 = offset << 3;

    let third: u8 = scale8(offset8, 85);

    let rgb: ColorRGB = match (hue & 0b1110_0000) >> 5 {
        0b000 => ColorRGB::new(255 - third, third, 0),
        0b001 => ColorRGB::new(171, 85 + third, 0),
        0b010 => {
            let two_thirds = scale8(offset8, ((256u16 * 2) / 3) as u8);
            ColorRGB::new(171 - two_thirds, 170 + third, 0)
        }
        0b011 => ColorRGB::new(0, 255 - third, third),
        0b100 => {
            let two_thirds = scale8(offset8, ((256u16 * 2) / 3) as u8);
            ColorRGB::new(0, 171 - two_thirds, 85 + two_thirds)
        }
        0b101 => ColorRGB::new(third, 0, 255 - third),
        0b110 => ColorRGB::new(85 + third, 0, 171 - third),
        0b111 => ColorRGB::new(170 + third, 0, 85 - third),
        _ => unsafe {unreachable_unchecked()}
    };

    rgb
}

/// Represents a color encoded in {hue, saturation, value} format.
#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub struct HSV {
    pub h: u8,
    pub s: u8,
    pub v: u8,
}

impl HSV {
    /// Grabs the hue component of the `HSV`.
    #[inline(always)]
    pub fn h(&self) -> u8 {
        self.h
    }
    /// Grabs the saturation component of the `HSV`.
    #[inline(always)]
    pub fn s(&self) -> u8 {
        self.s
    }
    /// Grabs the value component of the `HSV`.
    #[inline(always)]
    pub fn v(&self) -> u8 {
        self.v
    }
    /// Grabs the hue component of the `HSV`.
    #[inline(always)]
    pub fn hue(&self) -> u8 {
        self.h()
    }
    /// Grabs the saturation component of the `HSV`.
    #[inline(always)]
    pub fn saturation(&self) -> u8 {
        self.s()
    }
    /// Grabs the value component of the `HSV`.
    #[inline(always)]
    pub fn value(&self) -> u8 {
        self.v()
    }
    /// Sets the hue component of the `HSV`.
    #[inline(always)]
    pub fn set_hue(&mut self, h: u8) {
        self.h = h;
    }
    /// Sets the saturation component of the `HSV`.
    #[inline(always)]
    pub fn set_saturation(&mut self, s: u8) {
        self.s = s;
    }
    /// Sets the value component of the `HSV`.
    #[inline(always)]
    pub fn set_value(&mut self, v: u8) {
        self.v = v;
    }
}

impl fmt::Display for HSV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.h, self.s, self.v)
    }
}

impl From<(u8, u8, u8)> for HSV {
    #[inline(always)]
    fn from(other: (u8, u8, u8)) -> Self {
        Self::new(other.0, other.1, other.2)
    }
}

impl From<[u8; 3]> for HSV {
    #[inline(always)]
    fn from(other: [u8; 3]) -> Self {
        Self::new(other[0], other[1], other[2])
    }
}

impl HSV {
    /// Blank `HSV` object where all values are initialized to zero.
    pub const BLANK: HSV = HSV {h: 0, s: 0, v: 0};

    /// Create a new `HSV` object.
    #[inline(always)]
    pub const fn new(h: u8, s: u8, v: u8) -> Self {
        HSV { h, s, v }
    }

    /// Converts hue, saturation, and value to a `ColorRGB` using a visually balanced rainbow.
    pub fn to_rgb_rainbow(self) -> ColorRGB {
        let hue: u8 = self.h;
        let sat: u8 = self.s;
        let val: u8 = self.v;

        if sat == 0 {
            return ColorRGB::new(255, 255, 255);
        } else if val == 0 {
            return ColorRGB::new(0, 0, 0);
        }

        let mut rgb: ColorRGB = hue_to_full_rgb(hue);

        if sat != 255 {
            // Already checked for sat == 0;
            rgb.modify_all(|c| scale8(c, sat));
            let desat = 255 - sat;
            let brightness_floor = scale8(desat, desat);
            rgb.modify_all(|c| c + brightness_floor);
        }

        if val != 255 {
            // Already checked for val == 0
            rgb.modify_all(|c| scale8(c, val));
        }

        rgb
    }

    // Mathematical rainbow
    fn to_rgb_spectrum(self) -> ColorRGB {
        let mut hsv = self.clone();
        hsv.h = scale8(hsv.h, 191);
        unsafe { hsv.to_rgb_raw() }
    }

    // TODO: Test this!
    // Value can only be up to 191
    unsafe fn to_rgb_raw(self) -> ColorRGB {
        let value: u8 = self.v;
        let saturation: u8 = self.s;
        // The brightness floor is minimum number that all of
        // R, G, and B will be set to.
        let invsat: u8 = 255 - saturation;
        let brightness_floor: u8 = ((value as u16 * invsat as u16) / 256) as u8;

        // The color amplitude is the maximum amount of R, G, and B
        // that will be added on top of the brightness_floor to
        // create the specific hue desired.
        let color_amplitude: u8 = value - brightness_floor;

        // Figure out which section of the hue wheel we're in,
        // and how far offset we are withing that section
        let section: u8 = self.hue() / HSV_SECTION_3; // 0..2
        let offset: u8 = self.hue() % HSV_SECTION_3; // 0..63
        let rampup: u8 = offset; // 0..63
        let rampdown: u8 = (HSV_SECTION_3 - 1) - offset; // 63..0

        // compute color-amplitude-scaled-down versions of rampup and rampdown
        let rampup_amp_adj: u8 = ((rampup as u16 * color_amplitude as u16) / (256u16 / 4)) as u8;
        let rampdown_amp_adj: u8 =
            ((rampdown as u16 * color_amplitude as u16) / (256u16 / 4)) as u8;

        // add brightness_floor offset to everything
        let rampup_adj_with_floor: u8 = rampup_amp_adj + brightness_floor;
        let rampdown_adj_with_floor: u8 = rampdown_amp_adj + brightness_floor;

        if section == 0 {
            ColorRGB::new(
                brightness_floor,
                rampdown_adj_with_floor,
                rampup_adj_with_floor,
            )
        } else if section == 1 {
            ColorRGB::new(
                rampup_adj_with_floor,
                brightness_floor,
                rampdown_adj_with_floor,
            )
        } else {
            ColorRGB::new(
                rampdown_adj_with_floor,
                rampup_adj_with_floor,
                brightness_floor,
            )
        }
    }

    /// Changes the brightness (the value component in HSV) to it's maximum, 255.
    pub fn maximize_brightness(&mut self) {
        self.v = 255;
    }
}

#[cfg(test)]
mod test {
    use crate::{HSV, ColorRGB};

    fn into_360(n: u8) -> u32 {
        ((n as u32) * 360) >> 8
    }

    #[test]
    fn hsv2rgb_rainbow_6h() {
        for h in (0..=255).step_by(42) {
            let hsv = HSV::new(h as u8, 255, 255);
            let rgb: ColorRGB = ColorRGB::from(hsv);
        }
    }

    #[test]
    fn hsv2rgb_rainbow_256h() {
        for h in (0..=255).step_by(1) {
            let hsv = HSV::new(h as u8, 255, 255);
            let rgb: ColorRGB = ColorRGB::from(hsv);

        }
    }

    #[test]
    fn hsv2rgb_rainbow_all() {
        for h in (0..=255).step_by(1) {
            for s in (0..=255).step_by(15) {
                for v in (0..=255).step_by(15) {
                    let hsv = HSV::new(h as u8, s as u8, v as u8);
                    let rgb: ColorRGB = ColorRGB::from(hsv);
                    //println!("hsv ({},{},{}) -> r: {}, g: {}, b: {}\n", h, s, v, rgb.r, rgb.g, rgb.b);
                }
            }
        }
    }
}


