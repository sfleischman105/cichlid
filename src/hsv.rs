#[cfg(feature="no-std")]
use core::mem::transmute;
#[cfg(not(feature="no-std"))]
use std::mem::transmute;

use crate::ColorRGB;
use crate::scale::*;

const HSV_SECTION_3: u8 = 0x40;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct HSV {
    pub h: u8,
    pub s: u8,
    pub v: u8,
}

impl HSV {
    #[inline(always)]
    pub fn h(&self) -> u8 {
        self.h
    }
    #[inline(always)]
    pub fn s(&self) -> u8 {
        self.s
    }
    #[inline(always)]
    pub fn v(&self) -> u8 {
        self.v
    }
    #[inline(always)]
    pub fn hue(&self) -> u8 {
        self.h()
    }
    #[inline(always)]
    pub fn saturation(&self) -> u8 {
        self.s()
    }
    #[inline(always)]
    pub fn value(&self) -> u8 {
        self.v()
    }
    #[inline(always)]
    pub fn set_hue(&mut self, h: u8) {
        self.h = h;
    }
    #[inline(always)]
    pub fn set_saturation(&mut self, s: u8) {
        self.s = s;
    }
    #[inline(always)]
    pub fn set_value(&mut self, v: u8) {
        self.v = v;
    }
}

impl From<(u8, u8, u8)> for HSV {
    #[inline]
    fn from(other: (u8, u8, u8)) -> Self {
        unsafe { transmute(other) }
    }
}

impl From<[u8; 3]> for HSV {
    #[inline]
    fn from(other: [u8; 3]) -> Self {
        unsafe { transmute(other) }
    }
}

impl HSV {
    #[inline]
    pub fn new(h: u8, s: u8, v: u8) -> Self {
        Self::from((h, s, v))
    }

    // Full rainbow
    pub fn to_rgb_rainbow(&self) -> ColorRGB {
        let hue: u8 = self.h;
        let sat: u8 = self.s;
        let val: u8 = self.v;
        let offset: u8 = hue & 0x1F;
        let mut offset8 = offset;
        offset8 <<= 3;

        let third: u8 = scale8(offset8, 85);

        let mut rgb: ColorRGB = match (hue & 0b1110_000) >> 3 {
            0b000 => ColorRGB::new(255 - third, third, 0),
            0b001 => ColorRGB::new(171, 85 + third, 0),
            0b010 => {
                let two_thirds = scale8(offset8, ((256u16 * 2) / 3) as u8);
                ColorRGB::new(171 - two_thirds, 170 + third, 0)
            }
            0b011 => ColorRGB::new(0, 255 - third, third),
            0b100 => {
                let two_thirds = scale8(offset8, ((256u16 * 2) / 3) as u8);
                ColorRGB::new(0, 171 - two_thirds, 85 - two_thirds)
            }
            0b101 => ColorRGB::new(third, 0, 255 - third),
            0b110 => ColorRGB::new(85 + third, 0, 171 - third),
            0b111 => ColorRGB::new(170 + third, 0, 85 - third),
            _ => unreachable!(),
        };

        if sat != 255 {
            if sat == 0 {
                rgb = ColorRGB::new(255, 255, 255);
            } else {
                rgb.modify_all(|c| scale8(c, sat));
                let desat = 255 - sat;
                let brightness_floor = scale8(desat, desat);
                rgb.modify_all(|c| c + brightness_floor);
            }
        }

        if val != 255 {
            if sat == 0 {
                rgb = ColorRGB::new(0, 0, 0);
            } else {
                rgb.modify_all(|c| scale8(c, val));
            }
        }

        rgb
    }

    // Mathematical rainbow
    pub fn to_rgb_spectrum(&self) -> ColorRGB {
        let mut hsv = self.clone();
        hsv.h = scale8(hsv.h, 191);
        unsafe { hsv.to_rgb_raw() }
    }

    // Value can only be up to 191
    pub unsafe fn to_rgb_raw(&self) -> ColorRGB {
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
}
