//! Color gradient functions and impls.
//!
//! Create's smooth transitions between any two colors for any number of steps.

#[cfg(feature="no-std")]
use core::ops::DerefMut;
#[cfg(not(feature="no-std"))]
use std::ops::DerefMut;

use std::iter::ExactSizeIterator;

use crate::{HSV, ColorRGB};
use crate::lerp::ThreePointLerp;
use crate::color_util::GradientDirection;

impl<'a, T, H: 'a> super::FillGradient for T
    where
        T: IntoIterator<Item=&'a mut H>,
        T::IntoIter : ExactSizeIterator,
        H: From<HSV> {
    fn fill_gradient(self, start: HSV, end: HSV, dir: GradientDirection) {
        let iter = self.into_iter();
        let length = iter.len();
        hsv_gradient(iter, length, start, end, dir);
    }
}

impl<'a, T, H: 'a> super::FillGradientFull for T
    where
        T: IntoIterator<Item=&'a mut H>,
        T::IntoIter : ExactSizeIterator + DoubleEndedIterator,
        H: From<HSV> {
    fn fill_gradient_full(self, start: HSV, end: HSV, dir: GradientDirection) {
        let mut iter = self.into_iter();
        if let Some(t) = iter.next_back() {
            *t = H::from(end);
        } else {
            return;
        }
        let length = iter.len();
        hsv_gradient(iter, length, start, end, dir);
    }
}

impl<'a, T, H: 'a> super::FillGradientRGB for T
    where
        T: IntoIterator<Item=&'a mut H>,
        T::IntoIter : ExactSizeIterator,
        H: From<ColorRGB> {
    fn fill_gradient_rgb(self, start: ColorRGB, end: ColorRGB) {
        let iter = self.into_iter();
        let length = iter.len();
        rgb_gradient(iter, length, start, end);
    }
}

impl<'a, T, H: 'a> super::FillGradientRGBFull for T
    where
        T: IntoIterator<Item=&'a mut H>,
        T::IntoIter : ExactSizeIterator + DoubleEndedIterator,
        H: From<ColorRGB> {
    fn fill_gradient_rgb_full(self, start: ColorRGB, end: ColorRGB) {
        let mut iter = self.into_iter();
        if let Some(t) = iter.next_back() {
            *t = H::from(end);
        } else {
            return;
        }
        let length = iter.len();
        rgb_gradient(iter, length, start, end);
    }
}

/// Creates a axial (two-color) gradient from the HSV values `start` to (exclusive) `end`.
///
/// This function will fill the array inclusive of the `start` HSV and exclusive of the `end` HSV.
/// This means that after completion, `output[output.len() - 1] will not be the end color, but
/// rather the interpolated color before `start`. If you need the `end` color to appear last, see
/// `hsv_gradient_inclusive_end`.
///
/// # Edge Cases
///
/// If `output` is empty, the operation returns immediately.
pub fn hsv_gradient<'a, C: 'a + From<HSV>, I: IntoIterator<Item=&'a mut C>>(output: I, length: usize, start: HSV, end: HSV, dir: GradientDirection) {
    if length == 0 {
        return;
    }

    let mut start: HSV = start;
    let mut end: HSV = end;

    if end.v == 0 || end.s == 0 {
        end.h = start.h;
    }

    if start.v == 0 || start.s == 0 {
        start.h = end.h;
    }

    let hue_distance: i16 = dir.into_hue_distance(start.h, end.h);

    let lerp: ThreePointLerp = ThreePointLerp::new()
        .set_lerp_from_distance(0, start.h, hue_distance)
        .set_lerp_from_diff(1, start.s, end.s)
        .set_lerp_from_diff(2, start.v, end.v)
        .modify_delta(|d| d / (length as i16))
        .modify_delta(|d| d.wrapping_mul(2));

    output.into_iter()
        .zip(lerp)
        .for_each(|(i, hsv)| *i = C::from(HSV::from(hsv)));
}


pub fn rgb_gradient<'a, C: 'a + From<ColorRGB>, I: IntoIterator<Item=&'a mut C>>(output: I, length: usize, start: ColorRGB, end: ColorRGB) {
    if length == 0 {
        return;
    }

    let lerp: ThreePointLerp = ThreePointLerp::new()
        .set_lerp_from_diff(0, start.r, end.r)
        .set_lerp_from_diff(1, start.g, end.g)
        .set_lerp_from_diff(2, start.b, end.b)
        .modify_delta(|d| d / (length as i16))
        .modify_delta(|d| d.wrapping_mul(2));

    output.into_iter()
          .zip(lerp)
          .for_each(|(i, rgb)| *i = C::from(ColorRGB::from(rgb)));
}



#[cfg(test)]
mod test {
    use super::*;
    use crate::color_util::*;
    use crate::{HSV};

     #[test]
    fn gradient_sweep_test() {
        let start: HSV = HSV::new(0, 100, 50);
        let end: HSV = HSV::new(100, 200, 100);
        let mut out: [HSV; 5] = [HSV::BLANK; 5];

        let dir = GradientDirection::Shortest;
        out.as_mut().fill_gradient(start, end, dir);
        assert_eq!(*out.last().unwrap(), HSV::new(80, 180, 90));
        out.as_mut().fill_gradient_full(start, end, dir);
        assert_eq!(*out.last().unwrap(), end);

        let dir = GradientDirection::Forward;
        out.as_mut().fill_gradient(start, end, dir);
        assert_eq!(*out.last().unwrap(), HSV::new(80, 180, 90));
        out.as_mut().fill_gradient_full(start, end, dir);
        assert_eq!(*out.last().unwrap(), end);

        let dir = GradientDirection::Backwards;
        out.as_mut().fill_gradient(start, end, dir);
        out.as_mut().fill_gradient_full(start, end, dir);
        assert_eq!(*out.last().unwrap(), end);

        let dir = GradientDirection::Longest;
        out.as_mut().fill_gradient(start, end, dir);
        out.as_mut().fill_gradient_full(start, end, dir);
        assert_eq!(*out.last().unwrap(), end);
    }
}