//! Contains the HSV (hue, saturation, value) representation of a color.

// Credit for most of these functions goes to the authors of the FastLED library.

#[cfg(feature = "no-std")]
use core::fmt;
#[cfg(not(feature = "no-std"))]
use std::fmt;

use crate::math::ScalingInt;
use crate::math::*;
use crate::ColorRGB;

const HSV_SECTION_3: u8 = 0x40;

/// Converts hue to RGB at full brightness and full saturation.
#[inline(always)]
pub fn hue_to_full_rgb(hue: u8) -> ColorRGB {
    hsv_inner::hue2rgb_rainbow(hue)
}

/// Represents a color encoded in `(hue, saturation, value)` format.
///
/// This structure is useful for a more human-centered approach to thinking
/// about color.
#[derive(Copy, Clone, Default, Eq, PartialEq, Debug, Hash)]
pub struct HSV {
    pub h: u8,
    pub s: u8,
    pub v: u8,
}

impl HSV {
    /// Blank `HSV` object where all values are initialized to zero.
    pub const BLANK: HSV = HSV { h: 0, s: 0, v: 0 };

    /// Create a new `HSV` object.
    #[inline(always)]
    pub const fn new(h: u8, s: u8, v: u8) -> Self {
        HSV { h, s, v }
    }

    /// Grabs the hue component of the `HSV`.
    #[inline(always)]
    pub fn hue(self) -> u8 {
        self.h
    }
    /// Grabs the saturation component of the `HSV`.
    #[inline(always)]
    pub fn saturation(self) -> u8 {
        self.s
    }
    /// Grabs the value component of the `HSV`.
    #[inline(always)]
    pub fn value(self) -> u8 {
        self.v
    }

    /// Converts hue, saturation, and value to a `ColorRGB` using a visually balanced rainbow.
    pub fn to_rgb_rainbow(self) -> ColorRGB {
        let (hue, sat, val) = (self.h, self.s, self.v);

        let mut rgb: ColorRGB = match (sat, val) {
            (0, _) => return RGB!(255, 255, 255),
            (_, 0) => return RGB!(0, 0, 0),
            _ => hue_to_full_rgb(hue),
        };

        if sat != 255 {
            rgb.scale(sat);
            let desat: u8 = 255 - sat;
            let brightness_floor: u8 = desat.dim_raw();
            rgb += brightness_floor;
        }

        if val != 255 {
            rgb.scale(val);
        }

        rgb
    }

    /// Converts a `HSV` to a `ColorRGB` using a traditional Mathematical rainbow.
    pub fn to_rgb_spectrum(self) -> ColorRGB {
        let mut hsv = self;
        hsv.v = scale8(hsv.v, 191);
        unsafe { hsv.to_rgb_raw() }
    }

    /// Converts a `HSV` to a `ColorRGB` using a traditional Mathematical rainbow.
    ///
    /// # Safety
    ///
    /// Value can only be up to 191, or else undefined behavior will follow.
    pub unsafe fn to_rgb_raw(self) -> ColorRGB {
        // Taken directly from FastLED, credit goes to them for this
        debug_assert!(self.v <= 191);
        let value: u8 = self.v;
        let saturation: u8 = self.s;
        // The brightness floor is minimum number that all of
        // R, G, and B will be set to.
        let invsat: u8 = 255 - saturation;
        let brightness_floor: u8 = ((u16::from(value) * u16::from(invsat)) / 256) as u8;

        // The color amplitude is the maximum amount of R, G, and B that will be added on
        // top of the brightness_floor to create the specific hue desired.
        let color_amp: u8 = value - brightness_floor;

        // Figure out which section of the hue wheel we're in,
        // and how far offset we are withing that section
        let section: u8 = self.hue() / HSV_SECTION_3; // 0..2
        let offset: u8 = self.hue() % HSV_SECTION_3; // 0..63
        let rampup: u8 = offset; // 0..63
        let rampdown: u8 = (HSV_SECTION_3 - 1) - offset; // 63..0

        // compute color-amplitude-scaled-down versions of rampup and rampdown
        let rampup_amp_adj: u8 = ((u16::from(rampup) * u16::from(color_amp)) / (256u16 / 4)) as u8;
        let rampdown_amp_adj: u8 =
            ((u16::from(rampdown) * u16::from(color_amp)) / (256u16 / 4)) as u8;

        // add brightness_floor offset to everything
        let rampup_adj_with_floor: u8 = rampup_amp_adj + brightness_floor;
        let rampdown_adj_with_floor: u8 = rampdown_amp_adj + brightness_floor;

        match section {
            0 => RGB!(
                brightness_floor,
                rampdown_adj_with_floor,
                rampup_adj_with_floor
            ),
            1 => RGB!(
                rampup_adj_with_floor,
                brightness_floor,
                rampdown_adj_with_floor
            ),
            _ => RGB!(
                rampdown_adj_with_floor,
                rampup_adj_with_floor,
                brightness_floor
            ),
        }
    }

    /// Changes the brightness (the value component in HSV) to it's maximum, 255.
    pub fn maximize_brightness(&mut self) {
        self.v = 255;
    }
}

// Compacted function for low memory usage
#[cfg(feature = "low-mem")]
mod hsv_inner {
    use crate::math::scale::scale8;
    use crate::ColorRGB;
    #[cfg(feature = "no-std")]
    use core::hint::unreachable_unchecked;
    #[cfg(not(feature = "no-std"))]
    use std::hint::unreachable_unchecked;

    pub fn hue2rgb_rainbow(hue: u8) -> ColorRGB {
        let offset: u8 = hue & 0x1F;
        let offset8 = offset << 3;

        let third: u8 = scale8(offset8, 85);

        let rgb: ColorRGB = match (hue & 0b1110_0000) >> 5 {
            0b000 => RGB!(255 - third, third, 0),
            0b001 => RGB!(171, 85 + third, 0),
            0b010 => {
                let two_thirds = scale8(offset8, ((256u16 * 2) / 3) as u8);
                RGB!(171 - two_thirds, 170 + third, 0)
            }
            0b011 => RGB!(0, 255 - third, third),
            0b100 => {
                let two_thirds = scale8(offset8, ((256u16 * 2) / 3) as u8);
                RGB!(0, 171 - two_thirds, 85 + two_thirds)
            }
            0b101 => RGB!(third, 0, 255 - third),
            0b110 => RGB!(85 + third, 0, 171 - third),
            0b111 => RGB!(170 + third, 0, 85 - third),
            _ => unsafe { unreachable_unchecked() },
        };
        rgb
    }
}

#[cfg(not(feature = "low-mem"))]
mod hsv_inner {
    use crate::ColorRGB;

    #[inline(always)]
    pub fn hue2rgb_rainbow(hue: u8) -> ColorRGB {
        unsafe { *HSV_2_RGB_RAINBOW.get_unchecked(hue as usize) }
    }

    static HSV_2_RGB_RAINBOW: [ColorRGB; 256] = [
        RGB!(255, 0, 0),
        RGB!(253, 2, 0),
        RGB!(250, 5, 0),
        RGB!(247, 8, 0),
        RGB!(245, 10, 0),
        RGB!(242, 13, 0),
        RGB!(239, 16, 0),
        RGB!(237, 18, 0),
        RGB!(234, 21, 0),
        RGB!(231, 24, 0),
        RGB!(229, 26, 0),
        RGB!(226, 29, 0),
        RGB!(223, 32, 0),
        RGB!(221, 34, 0),
        RGB!(218, 37, 0),
        RGB!(215, 40, 0),
        RGB!(212, 43, 0),
        RGB!(210, 45, 0),
        RGB!(207, 48, 0),
        RGB!(204, 51, 0),
        RGB!(202, 53, 0),
        RGB!(199, 56, 0),
        RGB!(196, 59, 0),
        RGB!(194, 61, 0),
        RGB!(191, 64, 0),
        RGB!(188, 67, 0),
        RGB!(186, 69, 0),
        RGB!(183, 72, 0),
        RGB!(180, 75, 0),
        RGB!(178, 77, 0),
        RGB!(175, 80, 0),
        RGB!(172, 83, 0),
        RGB!(171, 85, 0),
        RGB!(171, 87, 0),
        RGB!(171, 90, 0),
        RGB!(171, 93, 0),
        RGB!(171, 95, 0),
        RGB!(171, 98, 0),
        RGB!(171, 101, 0),
        RGB!(171, 103, 0),
        RGB!(171, 106, 0),
        RGB!(171, 109, 0),
        RGB!(171, 111, 0),
        RGB!(171, 114, 0),
        RGB!(171, 117, 0),
        RGB!(171, 119, 0),
        RGB!(171, 122, 0),
        RGB!(171, 125, 0),
        RGB!(171, 128, 0),
        RGB!(171, 130, 0),
        RGB!(171, 133, 0),
        RGB!(171, 136, 0),
        RGB!(171, 138, 0),
        RGB!(171, 141, 0),
        RGB!(171, 144, 0),
        RGB!(171, 146, 0),
        RGB!(171, 149, 0),
        RGB!(171, 152, 0),
        RGB!(171, 154, 0),
        RGB!(171, 157, 0),
        RGB!(171, 160, 0),
        RGB!(171, 162, 0),
        RGB!(171, 165, 0),
        RGB!(171, 168, 0),
        RGB!(171, 170, 0),
        RGB!(166, 172, 0),
        RGB!(161, 175, 0),
        RGB!(155, 178, 0),
        RGB!(150, 180, 0),
        RGB!(145, 183, 0),
        RGB!(139, 186, 0),
        RGB!(134, 188, 0),
        RGB!(129, 191, 0),
        RGB!(123, 194, 0),
        RGB!(118, 196, 0),
        RGB!(113, 199, 0),
        RGB!(107, 202, 0),
        RGB!(102, 204, 0),
        RGB!(97, 207, 0),
        RGB!(91, 210, 0),
        RGB!(86, 213, 0),
        RGB!(81, 215, 0),
        RGB!(75, 218, 0),
        RGB!(70, 221, 0),
        RGB!(65, 223, 0),
        RGB!(59, 226, 0),
        RGB!(54, 229, 0),
        RGB!(49, 231, 0),
        RGB!(43, 234, 0),
        RGB!(38, 237, 0),
        RGB!(33, 239, 0),
        RGB!(27, 242, 0),
        RGB!(22, 245, 0),
        RGB!(17, 247, 0),
        RGB!(11, 250, 0),
        RGB!(6, 253, 0),
        RGB!(0, 255, 0),
        RGB!(0, 253, 2),
        RGB!(0, 250, 5),
        RGB!(0, 247, 8),
        RGB!(0, 245, 10),
        RGB!(0, 242, 13),
        RGB!(0, 239, 16),
        RGB!(0, 237, 18),
        RGB!(0, 234, 21),
        RGB!(0, 231, 24),
        RGB!(0, 229, 26),
        RGB!(0, 226, 29),
        RGB!(0, 223, 32),
        RGB!(0, 221, 34),
        RGB!(0, 218, 37),
        RGB!(0, 215, 40),
        RGB!(0, 212, 43),
        RGB!(0, 210, 45),
        RGB!(0, 207, 48),
        RGB!(0, 204, 51),
        RGB!(0, 202, 53),
        RGB!(0, 199, 56),
        RGB!(0, 196, 59),
        RGB!(0, 194, 61),
        RGB!(0, 191, 64),
        RGB!(0, 188, 67),
        RGB!(0, 186, 69),
        RGB!(0, 183, 72),
        RGB!(0, 180, 75),
        RGB!(0, 178, 77),
        RGB!(0, 175, 80),
        RGB!(0, 172, 83),
        RGB!(0, 171, 85),
        RGB!(0, 166, 90),
        RGB!(0, 161, 95),
        RGB!(0, 155, 101),
        RGB!(0, 150, 106),
        RGB!(0, 145, 111),
        RGB!(0, 139, 117),
        RGB!(0, 134, 122),
        RGB!(0, 129, 127),
        RGB!(0, 123, 133),
        RGB!(0, 118, 138),
        RGB!(0, 113, 143),
        RGB!(0, 107, 149),
        RGB!(0, 102, 154),
        RGB!(0, 97, 159),
        RGB!(0, 91, 165),
        RGB!(0, 86, 170),
        RGB!(0, 81, 175),
        RGB!(0, 75, 181),
        RGB!(0, 70, 186),
        RGB!(0, 65, 191),
        RGB!(0, 59, 197),
        RGB!(0, 54, 202),
        RGB!(0, 49, 207),
        RGB!(0, 43, 213),
        RGB!(0, 38, 218),
        RGB!(0, 33, 223),
        RGB!(0, 27, 229),
        RGB!(0, 22, 234),
        RGB!(0, 17, 239),
        RGB!(0, 11, 245),
        RGB!(0, 6, 250),
        RGB!(0, 0, 255),
        RGB!(2, 0, 253),
        RGB!(5, 0, 250),
        RGB!(8, 0, 247),
        RGB!(10, 0, 245),
        RGB!(13, 0, 242),
        RGB!(16, 0, 239),
        RGB!(18, 0, 237),
        RGB!(21, 0, 234),
        RGB!(24, 0, 231),
        RGB!(26, 0, 229),
        RGB!(29, 0, 226),
        RGB!(32, 0, 223),
        RGB!(34, 0, 221),
        RGB!(37, 0, 218),
        RGB!(40, 0, 215),
        RGB!(43, 0, 212),
        RGB!(45, 0, 210),
        RGB!(48, 0, 207),
        RGB!(51, 0, 204),
        RGB!(53, 0, 202),
        RGB!(56, 0, 199),
        RGB!(59, 0, 196),
        RGB!(61, 0, 194),
        RGB!(64, 0, 191),
        RGB!(67, 0, 188),
        RGB!(69, 0, 186),
        RGB!(72, 0, 183),
        RGB!(75, 0, 180),
        RGB!(77, 0, 178),
        RGB!(80, 0, 175),
        RGB!(83, 0, 172),
        RGB!(85, 0, 171),
        RGB!(87, 0, 169),
        RGB!(90, 0, 166),
        RGB!(93, 0, 163),
        RGB!(95, 0, 161),
        RGB!(98, 0, 158),
        RGB!(101, 0, 155),
        RGB!(103, 0, 153),
        RGB!(106, 0, 150),
        RGB!(109, 0, 147),
        RGB!(111, 0, 145),
        RGB!(114, 0, 142),
        RGB!(117, 0, 139),
        RGB!(119, 0, 137),
        RGB!(122, 0, 134),
        RGB!(125, 0, 131),
        RGB!(128, 0, 128),
        RGB!(130, 0, 126),
        RGB!(133, 0, 123),
        RGB!(136, 0, 120),
        RGB!(138, 0, 118),
        RGB!(141, 0, 115),
        RGB!(144, 0, 112),
        RGB!(146, 0, 110),
        RGB!(149, 0, 107),
        RGB!(152, 0, 104),
        RGB!(154, 0, 102),
        RGB!(157, 0, 99),
        RGB!(160, 0, 96),
        RGB!(162, 0, 94),
        RGB!(165, 0, 91),
        RGB!(168, 0, 88),
        RGB!(170, 0, 85),
        RGB!(172, 0, 83),
        RGB!(175, 0, 80),
        RGB!(178, 0, 77),
        RGB!(180, 0, 75),
        RGB!(183, 0, 72),
        RGB!(186, 0, 69),
        RGB!(188, 0, 67),
        RGB!(191, 0, 64),
        RGB!(194, 0, 61),
        RGB!(196, 0, 59),
        RGB!(199, 0, 56),
        RGB!(202, 0, 53),
        RGB!(204, 0, 51),
        RGB!(207, 0, 48),
        RGB!(210, 0, 45),
        RGB!(213, 0, 42),
        RGB!(215, 0, 40),
        RGB!(218, 0, 37),
        RGB!(221, 0, 34),
        RGB!(223, 0, 32),
        RGB!(226, 0, 29),
        RGB!(229, 0, 26),
        RGB!(231, 0, 24),
        RGB!(234, 0, 21),
        RGB!(237, 0, 18),
        RGB!(239, 0, 16),
        RGB!(242, 0, 13),
        RGB!(245, 0, 10),
        RGB!(247, 0, 8),
        RGB!(250, 0, 5),
        RGB!(253, 0, 2),
    ];
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

#[cfg(test)]
mod test {
    use crate::{ColorRGB, HSV};

    #[test]
    fn hsv2rgb_rainbow_6h() {
        for h in (0..=255).step_by(42) {
            let hsv = HSV::new(h as u8, 255, 255);
            let _rgb: ColorRGB = ColorRGB::from(hsv);
        }
    }

    #[test]
    fn hsv2rgb_rainbow_256h() {
        for h in (0..=255).step_by(1) {
            let hsv = HSV::new(h as u8, 255, 255);
            let _rgb: ColorRGB = ColorRGB::from(hsv);
        }
    }

    #[test]
    fn hsv2rgb_raw_256h() {
        for h in (0..=255).step_by(1) {
            for s in (0..=255).step_by(15) {
                for v in (0..=255).step_by(15) {
                    let hsv = HSV::new(h as u8, s as u8, v as u8);
                    let _rgb: ColorRGB = hsv.to_rgb_spectrum();
                }
            }
        }
    }

    #[test]
    fn hsv2rgb_rainbow_all() {
        for h in (0..=255).step_by(1) {
            for s in (0..=255).step_by(15) {
                for v in (0..=255).step_by(15) {
                    let hsv = HSV::new(h as u8, s as u8, v as u8);
                    let _rgb: ColorRGB = ColorRGB::from(hsv);
                }
            }
        }
    }
}
