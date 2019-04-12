//! Various Functions and traits for colors.
//!
//! The majority of these traits are not intended to be implemented by users. Rather, they are
//! meant for allowing easy ways to fill iterators with color.

pub mod gradient;
pub mod rainbow;

use crate::{ColorRGB, HSV};

/// Useful methods when iterating over `ColorRGB`s.
pub trait ColorIterMut: Sized {
    fn fill(self, color: ColorRGB);

    /// Sets all colors to Black.
    #[inline]
    fn clear(self) {
        self.fill(RGB!(0, 0, 0));
    }

    /// Fades all colors to black by the fractional component.
    fn fade_to_black(self, fade_by: u8);

    /// Blurs colors by `blur_amount`.
    ///
    /// A lower `blur_amount` means a less extreme blur. For example, a `blur_amount` of 64
    /// is a moderate blur, while past 171 the blur is somewhat flickery.
    ///
    /// This method does not retain brightness. Blurring will slowly fade all the colors to black.
    fn blur(self, blur_amount: u8);
}

/// Fills an iterable object with a gradient from the `HSV` values `start` to `finish`, exclusive of the
/// `finish`.
pub trait GradientFill {
    /// Fills a gradient from two HSV's using linear interpolation between the two.
    fn gradient_fill(self, start: HSV, end: HSV, dir: GradientDirection);
}

/// Fills an iterable object with a gradient from the `HSV` values `start` to `finish`, inclusive of the
/// `finish`.
pub trait GradientFillToInclusive {
    /// Fills a gradient from two HSV's using linear interpolation between the two, inclusive of
    /// the end HSV.
    fn gradient_fill_to_inclusive(self, start: HSV, end: HSV, dir: GradientDirection);
}

/// Fills an iterable object with a gradient from the `ColorRGB` values `start` to `finish`, exclusive of the
/// `finish`.
pub trait GradientFillRGB {
    /// Fills a gradient from two RGBs's using linear interpolation between the two.
    fn gradient_fill_rgb(self, start: ColorRGB, end: ColorRGB);
}

/// Fills an iterable object with a gradient from the `ColorRGB` values `start` to `finish`, inclusive of the
/// `finish`.
pub trait GradientFillRGBToInclusive {
    /// Fills a gradient from two RGB's using linear interpolation between the two, inclusive of
    /// the end RGB.
    fn gradient_fill_to_inclusive(self, start: ColorRGB, end: ColorRGB);
}

/// Fills an iterable object with a rainbow hue of a desired step size.
///
/// Step sizes are unsigned integers `u8`, `u16`, or `u32`. The Most significant byte of
/// each integer is used to represent the full number of hues to increment between each iterated
/// value, while the other bytes (if present) are added as a fractional component.
pub trait RainbowFill<C>: Sized {
    /// Fills an object with a rainbow gradient hue of a desired step size and from a desired
    /// starting hue.
    #[inline(always)]
    fn rainbow_fill(self, start_hue: u8, hue_delta: C) {
        self.rainbow_fill_with_sat_val(start_hue, hue_delta, 255, 255);
    }

    /// Fills an object with a rainbow gradient hue of a desired step size and from a desired
    /// starting hue and constant additional saturation and value (components of a HSV).
    fn rainbow_fill_with_sat_val(self, start_hue: u8, hue_delta: C, sat: u8, val: u8);
}

/// Fills an iterable object with a single complete rainbow.
///
/// If the the rainbow is needed backwards, try calling `iter.rev()` before calling this
/// method.
pub trait RainbowFillSingleCycle {
    fn rainbow_fill_single_cycle(self, start_hue: u8);
}

impl<'a, T> ColorIterMut for T
    where
        T: Sized + IntoIterator<Item = &'a mut ColorRGB>,
{
    fn fill(self, color: ColorRGB) {
        self.into_iter().for_each(|p| *p = color);
    }

    fn fade_to_black(self, fade_by: u8) {
        let fade_scale = 255 - fade_by;
        self.into_iter().for_each(|p| p.scale(fade_scale));
    }

    fn blur(self, blur_amount: u8) {
        let keep: u8 = 255 - blur_amount;
        let seep: u8 = blur_amount >> 1;
        let mut carry: ColorRGB = ColorRGB::Black;
        let mut iter = self.into_iter().peekable();
        loop {
            let cur = iter.next();
            let nxt = iter.peek();
            if let Some(i) = cur {
                let mut cur: ColorRGB = *i;
                cur.scale(keep);
                cur += carry;
                if let Some(nxt) = nxt {
                    let mut part: ColorRGB = **nxt;
                    part.scale(seep);
                    cur += part;
                    carry = part;
                }
                *i = cur;
            } else {
                break;
            }
        }
    }
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
            }
            GradientDirection::Longest => {
                if hue_diff < 128 {
                    HueDirection::Backwards
                } else {
                    HueDirection::Forward
                }
            }
            GradientDirection::Forward => HueDirection::Forward,
            GradientDirection::Backwards => HueDirection::Backwards,
        }
    }

    /// Returns the difference between hues.
    #[inline(always)]
    fn into_hue_distance(self, start_hue: u8, end_hue: u8) -> i16 {
        let hue_diff: u8 = end_hue.wrapping_sub(start_hue);
        match self.into_hue_direction(hue_diff) {
            HueDirection::Forward => i16::from(hue_diff) << 7,
            HueDirection::Backwards => {
                let hue_diff: u8 = (256u16).wrapping_sub(u16::from(hue_diff)) as u8;
                let hue_diff: i16 = i16::from(hue_diff) << 7;
                -hue_diff
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::HSV;

    #[test]
    fn blur_test() {
        let mut arr = [
            ColorRGB::Black,
            ColorRGB::Red,
            ColorRGB::BlueViolet,
            ColorRGB::Yellow,
        ];

        println!("{:?}", arr);
        for _ in 0..4 {
            arr.blur(64);
            println!("{:?}", arr);
        }
    }

    #[test]
    fn blur_test_long() {
        let mut arr = [ColorRGB::BlueViolet; 256];

        for _ in 0..4 {
            arr.blur(64);
        }
    }
}

