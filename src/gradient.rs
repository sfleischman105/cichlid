//! Color gradient functions.
//!
//! Create smooth transitions between any two colors for any number of steps.

#[cfg(feature="no-std")]
use core::mem::transmute;
#[cfg(not(feature="no-std"))]
use std::mem::transmute;

use crate::HSV;

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

/// Creates a axial (two-color) gradient from the HSV values `start` to `end`. This function
/// will fill the array inclusive of the `start` HSV and exclusive of the `end` HSV. This means
/// that after completion, `output[output.len() - 1] will not be the end color, but rather the
/// interpolated color before `start`. If you need the `end` color to appear last, see
/// `hsv_gradient_inclusive_end`
///
/// # Edge Cases
///
/// If `output` is empty, the operation returns immediately.
pub fn hsv_gradient<C: From<HSV>>(start: HSV, end: HSV, dir: GradientDirection, output: &mut [C]) {
    if output.is_empty() {
        return;
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
    let sat_distance: i16 = (end.s as i16 - start.s as i16).wrapping_shl(7);
    let val_distance: i16 = (end.v as i16 - start.v as i16).wrapping_shl(7);

    let hue_delta: i16 = hue_distance / (output.len() as i16);
    let sat_delta: i16 = sat_distance / (output.len() as i16);
    let val_delta: i16 = val_distance / (output.len() as i16);

    let hue_delta: i16 = hue_delta.wrapping_mul(2);
    let sat_delta: i16 = sat_delta.wrapping_mul(2);
    let val_delta: i16 = val_delta.wrapping_mul(2);

    let mut sat_accum: u16 = (start.s as u16) << 8;
    let mut val_accum: u16 = (start.v as u16) << 8;
    let mut hue_accum: u16 = (start.h as u16) << 8;

    for i in output.iter_mut() {
        let hsv = HSV::new((hue_accum >> 8) as u8,
                           (sat_accum >> 8) as u8,
                           (val_accum >> 8) as u8);
        *i = C::from(hsv);
        sat_accum = sat_accum.wrapping_add(unsafe{transmute::<i16,u16>(sat_delta)});
        val_accum = val_accum.wrapping_add(unsafe{transmute::<i16,u16>(val_delta)});
        hue_accum = hue_accum.wrapping_add(unsafe{transmute::<i16,u16>(hue_delta)});
    }
}

/// Creates a axial (two-color) gradient from the HSV values `start` to `end`. This function
/// will fill the array inclusive of the both the `start` and `end` HSV's. This means that after
/// completion, `output[output.len() - 1] will be the end color. If you need want a gradient
/// without the `end` color to appearing last, see `hsv_gradient`.
///
/// # Edge Cases
///
/// If `output` is empty, the operation returns immediately. If `output.len() == 1`, only the
/// `start` HSV will be left in the array.
pub fn hsv_gradient_inclusive_end<C: From<HSV>>(start: HSV, end: HSV,
                                          dir: GradientDirection, output: &mut [C]) {
    let len = output.len();
    if len > 1 {
        output[len - 1] = C::from(end);
    }
    let slice = &mut output[0..(len - 1)];
    hsv_gradient::<C>(start, end, dir, slice);
}


#[cfg(test)]
mod test {
    use crate::{HSV};
    use crate::gradient::{hsv_gradient, hsv_gradient_inclusive_end, GradientDirection};

    #[test]
    fn gradient_sweep_test() {
        let start: HSV = HSV::new(0, 100, 50);
        let end: HSV = HSV::new(100, 200, 100);
        let mut out: [HSV; 5] = [HSV::BLANK; 5];

        let dir = GradientDirection::Shortest;
        hsv_gradient(start, end, dir, &mut out);
        assert_eq!(*out.last().unwrap(), HSV::new(80, 180, 90));
        hsv_gradient_inclusive_end(start, end, dir, &mut out);
        assert_eq!(*out.last().unwrap(), end);

        let dir = GradientDirection::Forward;
        hsv_gradient(start, end, dir, &mut out);
        assert_eq!(*out.last().unwrap(), HSV::new(80, 180, 90));
        hsv_gradient_inclusive_end(start, end, dir, &mut out);
        assert_eq!(*out.last().unwrap(), end);

        let dir = GradientDirection::Backwards;
        hsv_gradient(start, end, dir, &mut out);
        hsv_gradient_inclusive_end(start, end, dir, &mut out);
        assert_eq!(*out.last().unwrap(), end);

        let dir = GradientDirection::Longest;
        hsv_gradient(start, end, dir, &mut out);
        hsv_gradient_inclusive_end(start, end, dir, &mut out);
        assert_eq!(*out.last().unwrap(), end);
    }
}