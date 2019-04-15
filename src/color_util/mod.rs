//! Various Functions and traits for colors.
//!
//! The majority of these traits are not intended to be implemented by users. Rather, they are
//! meant for allowing easy ways to fill iterators with color.
//!
//! ## Importing
//!
//! Importing can be done by one of the following ways:
//!
//! ```
//! use cichlid::*;
//! use cichlid::prelude::*;
//! use cichlid::color_util::ColorIterMut;
//! use cichlid::color_util::ColorIterMut as _;
//! ```
//!
//! It is preferable to import that traits anonymously with `use ... as _`. This is because
//! these traits are not meant to be implemented directly, so only the types implementing
//! the traits need to be imported.
//!
//! ## Traits
//!
//! - [`ColorIterMut`]:
//!     - General functions applying on iterators over [`ColorRGB`]'s.
//!     - Examples of functions: `blur()`, `fade_to_black()`, `fill()`.
//!     - Implemented for all Iterators over `&mut ColorRGB`.
//! - [`GradientFill`]:
//!     - Fills a Gradient from one [`HSV`] to another using Linear Interpolation.
//!     - Implemented for any iterators implementing `ExactSizeIter`.
//!     - Also Generic over `Iterator::Item` for any item implementing `From<HSV>`, meaning
//!       this method works on itorators over both `HSV` and `ColorRGB`.
//! - [`GradientFillToInclusive`]:
//!     - Same as `GradientFill`, but creates a gradient inclusive of the last `HSV`. This means
//!       that the last item iterated over will be garunteed to be the last `HSV`, rather than
//!       being the color before.
//!     - Generally Requires a `DoubleEndedIter` trait be implemented for implementee's type.
//! - [`GradientFillRGB`]:
//!     - Works the same as`GradientFill`, but does a linear interpolation between two `ColorRGBs`.
//!     - This results in a gradient that is a mathematically consistent transition, but isn't
//!       as visually pleasing compared to doing Linear Interpolation with `HSV`s.
//! - [`GradientFillRGBToInclusive`]:
//!     - The `GradientFillToInclusive` to `GradientFillRGB`, as it fills up to and including the
//!       last color.
//!     - Also requires that the Iterator implements `DoubleEndedIter`.
//! - [`RainbowFill`]:
//!     - Fills an Iterator over `&mut From<HSV>` with a rainbow pattern. This pattern repeats
//!       forever, starting at a specific hue and taking a user-defined step size for each new
//!       element.
//! - [`RainbowFillSingleCycle`]:
//!     - Fills an Iterator with a full rainbow cycle.
//!
//! [`ColorIterMut`]: ./trait.ColorIterMut.html
//! [`GradientFill`]: ./trait.GradientFill.html
//! [`GradientFillToInclusive`]: ./trait.GradientFillToInclusive.html
//! [`GradientFillRGB`]: ./trait.GradientFill.html
//! [`GradientFillRGBToInclusive`]: ./trait.GradientFillRGBToInclusive.html
//! [`RainbowFill`]: ./trait.RainbowFill.html
//! [`RainbowFillSingleCycle`]: ./trait.RainbowFillSingleCycle.html
//! [`ColorRGB`]: ../struct.ColorRGB.html
//! [`HSV`]: ../struct.HSV.html

pub mod gradient;
pub mod rainbow;

use crate::{ColorRGB, HSV};

/// Useful methods when iterating over `ColorRGB`s.
///
/// This is `impl`'d for any Iterator over `&mut RGB`. This includes both arrays and slices, the
/// most common use case for this.
///
/// # Examples
///
/// Operating Directly on an array of `ColorRGB`s:
///
/// ```
/// use cichlid::{prelude::*, ColorRGB};
///
/// let mut colors = [ColorRGB::BlanchedAlmond; 100];
///
/// colors.fill(ColorRGB::Yellow);
/// colors.iter().for_each(|c| assert_eq!(*c, ColorRGB::Yellow));
/// ```
///
/// Operating on slices is supported as well:
///
/// ```
/// use cichlid::{prelude::*, ColorRGB};
///
/// let mut colors = [ColorRGB::Purple; 50];
/// let color_slice = &mut colors[0..40];
///
/// color_slice.clear();
/// color_slice.iter().for_each(|c| assert_eq!(*c, ColorRGB::Black));
/// ```
pub trait ColorIterMut: Sized {
    /// Fills an entire Iterator with the specified color.
    fn fill(self, color: ColorRGB);

    /// Sets all colors to black.
    #[inline]
    fn clear(self) {
        self.fill(RGB!(0, 0, 0));
    }

    /// Fades all colors to black by the a fraction.
    ///
    /// The `fade_by` parameter is interpreted as a fraction with a denominator of 255,
    /// of which itself is the numerator.
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
///
/// # Examples
///
/// ```
/// use cichlid::{prelude::*, ColorRGB, HSV, GradientDirection};
///
/// let mut colors = [ColorRGB::Black; 24];
/// let start = HSV::new(0, 255, 255);
/// let end = HSV::new(100, 255, 180);
/// colors.gradient_fill(start, end, GradientDirection::Longest);
/// ```
///
/// Also usable over Iterators for `HSV`:
///
/// ```
/// use cichlid::{prelude::*, HSV, GradientDirection};
///
/// let mut colors = [HSV::BLANK; 80];
/// let start = HSV::new(0, 255, 255);
/// let end = HSV::new(100, 255, 180);
/// colors.gradient_fill(start, end, GradientDirection::Longest);
/// ```
pub trait GradientFill {
    /// Fills a gradient from two HSV's using linear interpolation between the two.
    fn gradient_fill(self, start: HSV, end: HSV, dir: GradientDirection);
}

/// Fills an iterable object with a gradient from the `HSV` values `start` to `finish`, inclusive of the
/// `finish`.
///
/// # Examples
///
/// ```
/// use cichlid::{prelude::*, ColorRGB, HSV, GradientDirection};
///
/// let mut colors = [ColorRGB::Black; 80];
/// let start = HSV::new(130, 200, 251);
/// let end = HSV::new(206, 100, 255);
///
/// colors.gradient_fill_to_inclusive(start, end, GradientDirection::Shortest);
/// assert_eq!(*colors.last().unwrap(), ColorRGB::from(end));
/// ```
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
    fn gradient_fill_rgb_to_inclusive(self, start: ColorRGB, end: ColorRGB);
}

/// Fills an iterable object with a rainbow hue of a desired step size.
///
/// Step sizes is a `u16`. The Most significant byte of each integer is used to represent the
/// full number of hues to increment between each iterated value, while the second (LSB)
/// byte is added as a fractional component.
///
/// For example, if one desires to change a single hue between each element, the `hue_delta`
/// should be set to `0x0100`. If a hue is desired to change every 256 elements, then the
/// `hue_delta` should be `0x0001`.
pub trait RainbowFill: Sized {
    /// Fills an object with a rainbow gradient hue of a desired step size and from a desired
    /// starting hue.
    #[inline(always)]
    fn rainbow_fill(self, start_hue: u8, hue_delta: u16) {
        self.rainbow_fill_with_sat_val(start_hue, hue_delta, 255, 255);
    }

    /// Fills an object with a rainbow gradient hue of a desired step size and from a desired
    /// starting hue and constant additional saturation and value (components of a HSV).
    fn rainbow_fill_with_sat_val(self, start_hue: u8, hue_delta: u16, sat: u8, val: u8);
}

/// Fills an iterable object with a single complete rainbow.
///
/// If the the rainbow is needed backwards, try calling `iter.rev()` before calling this
/// method.
pub trait RainbowFillSingleCycle {
    fn rainbow_fill_single_cycle(self, start_hue: u8);
}

impl<'a, T: Sized + IntoIterator<Item = &'a mut ColorRGB>> ColorIterMut for T {
    fn fill(self, color: ColorRGB) {
        self.into_iter().for_each(|p| *p = color);
    }

    fn fade_to_black(self, fade_by: u8) {
        self.into_iter().for_each(|p| p.fade_to_black_by(fade_by));
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

        for _ in 0..4 {
            arr.blur(64);
        }
    }

    #[test]
    fn slice_color_itermut_test() {
        let mut colors = [ColorRGB::Purple; 50];
        let color_slice = &mut colors[0..40];
        color_slice.blur(20);
        color_slice.clear();
        color_slice
            .iter()
            .for_each(|c| assert_eq!(*c, ColorRGB::Black));
    }

    #[test]
    fn color_itermut_test() {
        let mut colors = [ColorRGB::Gold; 50];
        for i in 0..=255 {
            colors.blur(i);
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
