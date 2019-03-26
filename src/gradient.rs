#[cfg(feature="no-std")]
use core::mem::transmute;
#[cfg(not(feature="no-std"))]
use std::mem::transmute;

use crate::{HSV};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GradientDirection {
    Shortest,  // Goes around the color wheel by the shortest direction
    Longest,   // Goes around the color wheel by longest direction
    Forward,   // Clockwise, Hue Increases (including wrap around)
    Backwards, // Counter-Clockwise; Hue Decreases (including wrap around)
}

impl GradientDirection {
    /// hue_diff = end_hue - start_hue
    #[inline]
    pub fn into_direction(self, hue_diff: u8) -> GradientDirection {
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

    #[inline]
    pub fn into_hue_distance(self, start_hue: u8, end_hue: u8) -> i16 {
        let hue_diff: u8 = end_hue.wrapping_sub(start_hue);
        let new_dir = self.into_direction(hue_diff);
        if new_dir == GradientDirection::Forward {
            (hue_diff as i16) << 7
        } else {
            ((256 - hue_diff as i16) << 7) * -1
        }
    }


}

pub fn gradient<C: From<HSV>>(mut start: HSV, mut end: HSV, dir: GradientDirection, output: &mut [C]) {
    if output.is_empty() {
        return;
    }

    if end.v == 0 || end.s == 0 {
        end.h = start.h;
    }

    if start.v == 0 || start.s == 0 {
        start.h = end.h;
    }

    let sat_distance: i16 = end.s as i16 - start.s as i16;
    let val_distance: i16 = end.v as i16 - start.v as i16;
    let hue_distance: i16 = dir.into_hue_distance(end.h, start.h);

    let sat_delta: i16 = sat_distance / (output.len() as i16);
    let val_delta: i16 = val_distance / (output.len() as i16);
    let hue_delta: i16 = hue_distance / (output.len() as i16);

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