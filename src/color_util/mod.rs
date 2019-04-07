pub mod gradient;
pub mod rainbow;

use crate::{HSV, ColorRGB};

pub trait FillGradient {
    fn fill_gradient(self, start: HSV, end: HSV, dir: GradientDirection);
}

pub trait FillGradientFull {
    fn fill_gradient_full(self, start: HSV, end: HSV, dir: GradientDirection);
}


pub trait FillGradientRGB {
    fn fill_gradient_rgb(self, start: ColorRGB, end: ColorRGB);
}

pub trait FillGradientRGBFull {
    fn fill_gradient_rgb_full(self, start: ColorRGB, end: ColorRGB);
}


pub trait FillRainbow<C> : Sized {
    #[inline(always)]
    fn fill_rainbow(self, start_hue: u8, hue_delta: C) {
        self.fill_rainbow_with_sat_val(start_hue, hue_delta, 255, 255);
    }

    fn fill_rainbow_with_sat_val(self, start_hue: u8, hue_delta: C, sat: u8, val: u8);
}

pub trait FillSingularRainbow {
    fn fill_singular_rainbow(self, start_hue: u8);
}

/// Possible Directions around the color wheel a hue can go.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum HueDirection {
    /// Goes around the color wheel clockwise. ala, Hue increases as the gradient progresses,
    /// including integer wrapping.
    Forward = 0,
    /// Goes around the color wheel counter-clockwise. Hue decreases as the gradient progresses,
    /// including integer wrapping.
    Backwards = 1,
}

/// Possible Directions around the color wheel a gradient can go.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum GradientDirection {
    /// Goes around the color wheel clockwise. ala, Hue increases as the gradient progresses,
    /// including integer wrapping.
    Forward = 0,
    /// Goes around the color wheel counter-clockwise. Hue decreases as the gradient progresses,
    /// including integer wrapping.
    Backwards = 1,
    /// Goes around the color wheel by the shortest direction available.
    Shortest = 2,
    /// Goes around the color wheel by longest direction available.
    Longest = 3,
}

impl GradientDirection {
    /// Transforms a `GradientDirection` into a `HueDirection`.
    ///
    /// `hue_diff` is the difference between the ending hue and the starting hue. Specifically,
    /// `hue_diff = end_hue.wrapping_sub(start_hue)`. This is needed in the cases where the
    /// discriminant is neither forwards or backwards.
    #[inline]
    pub fn into_hue_direction(self, hue_diff: u8) -> HueDirection {
        match self {
            GradientDirection::Shortest => {
                if hue_diff > 127 {
                    HueDirection::Backwards
                } else {
                    HueDirection::Forward
                }
            },
            GradientDirection::Longest => {
                if hue_diff < 128 {
                    HueDirection::Backwards
                } else {
                    HueDirection::Forward
                }
            },
            GradientDirection::Forward => HueDirection::Forward,
            GradientDirection::Backwards => HueDirection::Backwards,
        }
    }

    /// Returns the difference between hues.
    #[inline(always)]
    fn into_hue_distance(self, start_hue: u8, end_hue: u8) -> i16 {
        let hue_diff: u8 = end_hue.wrapping_sub(start_hue);
        match self.into_hue_direction(hue_diff) {
            HueDirection::Forward => (hue_diff as i16) << 7,
            HueDirection::Backwards => {
                let hue_diff: u8 = (256u16).wrapping_sub(hue_diff as u16) as u8;
                let hue_diff: i16 = (hue_diff as i16) << 7;
                hue_diff * -1
            }
        }
    }
}

impl From<HueDirection> for GradientDirection {
    fn from(dir: HueDirection) -> GradientDirection {
        match dir {
            HueDirection::Forward => GradientDirection::Forward,
            HueDirection::Backwards => GradientDirection::Backwards,
        }
    }
}