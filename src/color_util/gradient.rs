//! Color gradient functions.
//!
//! Create smooth transitions between any two colors for any number of steps.

#[cfg(feature="no-std")]
use core::ops::DerefMut;
#[cfg(not(feature="no-std"))]
use std::ops::DerefMut;

use crate::{HSV, ColorRGB};
use crate::lerp::ThreePointLerp;

pub trait FillGradient<AXEL, OUTPUT>
    where
        AXEL: Copy,
        OUTPUT: From<AXEL>,
        Self: DerefMut<Target=[OUTPUT]>
{

    fn fill_gradient_slice(arr: &mut [OUTPUT], start: AXEL, end: AXEL, dir: GradientDirection);

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
    fn fill_gradient(&mut self, start: AXEL, end: AXEL, dir: GradientDirection) {
        let slice: &mut [OUTPUT] = self.as_mut();
        Self::fill_gradient_slice(slice, start, end, dir);
    }

    fn fill_gradient_full(&mut self, start: AXEL, end: AXEL, dir: GradientDirection) {
        let arr: &mut [OUTPUT] = self.deref_mut();
        let len: usize = arr.len();
        if len > 1 {
            arr[len - 1] = OUTPUT::from(end);
        }
        let slice: &mut [OUTPUT] = &mut arr[0..(len - 1)];
        Self::fill_gradient_slice(slice, start, end, dir);
    }
}

impl<'a, O: From<HSV>, T: DerefMut<Target=[O]>> FillGradient<HSV, O> for T {
    fn fill_gradient_slice(arr: &mut [O], start: HSV, end: HSV, dir: GradientDirection) {
        hsv_gradient::<O>(start, end, dir, arr);
    }
}

impl<'a, O: From<ColorRGB>, T: DerefMut<Target=[O]>> FillGradient<ColorRGB, O> for T {
    fn fill_gradient_slice(arr: &mut [O], start: ColorRGB, end: ColorRGB, _: GradientDirection) {
        rgb_gradient::<O>(start, end, arr);
    }
}


/// Possible Directions around the color wheel a gradient can go.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GradientDirection {
    /// Goes around the color wheel by the shortest direction available.
    Shortest,
    /// Goes around the color wheel by longest direction available.
    Longest,
    /// Goes around the color wheel clockwise. ala, Hue increases as the gradient progresses,
    /// including integer wrapping.
    Forward,
    /// Goes around the color wheel counter-clockwise. Hue decreases as the gradient progresses,
    /// including integer wrapping.
    Backwards,
}

impl GradientDirection {
    /// Transforms `self` into a "real" direction. A real direction is defined as the two
    /// discriminants `GradientDirection::Forward` and `GradientDirection::Backwards`. If
    /// `self` is already one of these two, the same direction is returned. Otherwise
    /// if `self` is `GradientDirection::Shortest` or `GradientDirection::Longest`, it'll be
    /// converted into a "real" direction.
    ///
    /// `hue_diff` is the difference between the ending hue and the starting hue. Specifically,
    /// `hue_diff = end_hue.wrapping_sub(start_hue)`. This is needed for determining the output direction
    /// when the input direction is non-real.
    #[inline]
    pub fn force_direction(self, hue_diff: u8) -> GradientDirection {
        match self {
            GradientDirection::Shortest => {
                if hue_diff > 127 {
                    GradientDirection::Backwards
                } else {
                    GradientDirection::Forward
                }
            },
            GradientDirection::Longest => {
                if hue_diff < 128 {
                    GradientDirection::Backwards
                } else {
                    GradientDirection::Forward
                }
            },
            GradientDirection::Forward => self,
            GradientDirection::Backwards => self,
        }
    }

    /// Returns the difference between hues.
    #[inline(always)]
    fn into_hue_distance(self, start_hue: u8, end_hue: u8) -> i16 {
        let hue_diff: u8 = end_hue.wrapping_sub(start_hue);
        if self.force_direction(hue_diff) == GradientDirection::Forward {
            (hue_diff as i16) << 7
        } else {
            let hue_diff: u8 = (256u16).wrapping_sub(hue_diff as u16) as u8;
            let hue_diff: i16 = (hue_diff as i16) << 7;
            hue_diff * -1
        }
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
pub fn hsv_gradient<C: From<HSV>>(start: HSV, end: HSV, dir: GradientDirection, output: &mut [C]) {
    let len = output.len();
    match len {
        0 => return,
        1 => {
            output[0] = C::from(start);
            return;
        },
        _ => {}
    }

    let mut start = start;
    let mut end = end;

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
        .modify_delta(|d| d / (len as i16))
        .modify_delta(|d| d.wrapping_mul(2));

    output.iter_mut()
        .zip(lerp)
        .for_each(|(i, hsv)| *i = C::from(HSV::from(hsv)));
}


pub fn rgb_gradient<C: From<ColorRGB>>(start: ColorRGB, end: ColorRGB, output: &mut [C]) {
    let len = output.len();
    match len {
        0 => return,
        1 => {
            output[0] = C::from(start);
            return;
        },
        _ => {}
    }

    let lerp: ThreePointLerp = ThreePointLerp::new()
        .set_lerp_from_diff(0, start.r, end.r)
        .set_lerp_from_diff(1, start.g, end.g)
        .set_lerp_from_diff(2, start.b, end.b)
        .modify_delta(|d| d / (len as i16))
        .modify_delta(|d| d.wrapping_mul(2));

    output.iter_mut()
          .zip(lerp)
          .for_each(|(i, rgb)| *i = C::from(ColorRGB::from(rgb)));
}



#[cfg(test)]
mod test {
    use super::*;
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