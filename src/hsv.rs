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
fn hue_to_full_rgb(hue: u8) -> ColorRGB {
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
            (0, _) => return mk_rgb!(255, 255, 255),
            (_, 0) => return mk_rgb!(0, 0, 0),
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
        hsv.v = scale_u8(hsv.v, 191);
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
            0 => mk_rgb!(
                brightness_floor,
                rampdown_adj_with_floor,
                rampup_adj_with_floor
            ),
            1 => mk_rgb!(
                rampup_adj_with_floor,
                brightness_floor,
                rampdown_adj_with_floor
            ),
            _ => mk_rgb!(
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
            0b000 => mk_rgb!(255 - third, third, 0),
            0b001 => mk_rgb!(171, 85 + third, 0),
            0b010 => {
                let two_thirds = scale8(offset8, ((256u16 * 2) / 3) as u8);
                mk_rgb!(171 - two_thirds, 170 + third, 0)
            }
            0b011 => mk_rgb!(0, 255 - third, third),
            0b100 => {
                let two_thirds = scale8(offset8, ((256u16 * 2) / 3) as u8);
                mk_rgb!(0, 171 - two_thirds, 85 + two_thirds)
            }
            0b101 => mk_rgb!(third, 0, 255 - third),
            0b110 => mk_rgb!(85 + third, 0, 171 - third),
            0b111 => mk_rgb!(170 + third, 0, 85 - third),
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
        mk_rgb!(255, 0, 0),
        mk_rgb!(253, 2, 0),
        mk_rgb!(250, 5, 0),
        mk_rgb!(247, 8, 0),
        mk_rgb!(245, 10, 0),
        mk_rgb!(242, 13, 0),
        mk_rgb!(239, 16, 0),
        mk_rgb!(237, 18, 0),
        mk_rgb!(234, 21, 0),
        mk_rgb!(231, 24, 0),
        mk_rgb!(229, 26, 0),
        mk_rgb!(226, 29, 0),
        mk_rgb!(223, 32, 0),
        mk_rgb!(221, 34, 0),
        mk_rgb!(218, 37, 0),
        mk_rgb!(215, 40, 0),
        mk_rgb!(212, 43, 0),
        mk_rgb!(210, 45, 0),
        mk_rgb!(207, 48, 0),
        mk_rgb!(204, 51, 0),
        mk_rgb!(202, 53, 0),
        mk_rgb!(199, 56, 0),
        mk_rgb!(196, 59, 0),
        mk_rgb!(194, 61, 0),
        mk_rgb!(191, 64, 0),
        mk_rgb!(188, 67, 0),
        mk_rgb!(186, 69, 0),
        mk_rgb!(183, 72, 0),
        mk_rgb!(180, 75, 0),
        mk_rgb!(178, 77, 0),
        mk_rgb!(175, 80, 0),
        mk_rgb!(172, 83, 0),
        mk_rgb!(171, 85, 0),
        mk_rgb!(171, 87, 0),
        mk_rgb!(171, 90, 0),
        mk_rgb!(171, 93, 0),
        mk_rgb!(171, 95, 0),
        mk_rgb!(171, 98, 0),
        mk_rgb!(171, 101, 0),
        mk_rgb!(171, 103, 0),
        mk_rgb!(171, 106, 0),
        mk_rgb!(171, 109, 0),
        mk_rgb!(171, 111, 0),
        mk_rgb!(171, 114, 0),
        mk_rgb!(171, 117, 0),
        mk_rgb!(171, 119, 0),
        mk_rgb!(171, 122, 0),
        mk_rgb!(171, 125, 0),
        mk_rgb!(171, 128, 0),
        mk_rgb!(171, 130, 0),
        mk_rgb!(171, 133, 0),
        mk_rgb!(171, 136, 0),
        mk_rgb!(171, 138, 0),
        mk_rgb!(171, 141, 0),
        mk_rgb!(171, 144, 0),
        mk_rgb!(171, 146, 0),
        mk_rgb!(171, 149, 0),
        mk_rgb!(171, 152, 0),
        mk_rgb!(171, 154, 0),
        mk_rgb!(171, 157, 0),
        mk_rgb!(171, 160, 0),
        mk_rgb!(171, 162, 0),
        mk_rgb!(171, 165, 0),
        mk_rgb!(171, 168, 0),
        mk_rgb!(171, 170, 0),
        mk_rgb!(166, 172, 0),
        mk_rgb!(161, 175, 0),
        mk_rgb!(155, 178, 0),
        mk_rgb!(150, 180, 0),
        mk_rgb!(145, 183, 0),
        mk_rgb!(139, 186, 0),
        mk_rgb!(134, 188, 0),
        mk_rgb!(129, 191, 0),
        mk_rgb!(123, 194, 0),
        mk_rgb!(118, 196, 0),
        mk_rgb!(113, 199, 0),
        mk_rgb!(107, 202, 0),
        mk_rgb!(102, 204, 0),
        mk_rgb!(97, 207, 0),
        mk_rgb!(91, 210, 0),
        mk_rgb!(86, 213, 0),
        mk_rgb!(81, 215, 0),
        mk_rgb!(75, 218, 0),
        mk_rgb!(70, 221, 0),
        mk_rgb!(65, 223, 0),
        mk_rgb!(59, 226, 0),
        mk_rgb!(54, 229, 0),
        mk_rgb!(49, 231, 0),
        mk_rgb!(43, 234, 0),
        mk_rgb!(38, 237, 0),
        mk_rgb!(33, 239, 0),
        mk_rgb!(27, 242, 0),
        mk_rgb!(22, 245, 0),
        mk_rgb!(17, 247, 0),
        mk_rgb!(11, 250, 0),
        mk_rgb!(6, 253, 0),
        mk_rgb!(0, 255, 0),
        mk_rgb!(0, 253, 2),
        mk_rgb!(0, 250, 5),
        mk_rgb!(0, 247, 8),
        mk_rgb!(0, 245, 10),
        mk_rgb!(0, 242, 13),
        mk_rgb!(0, 239, 16),
        mk_rgb!(0, 237, 18),
        mk_rgb!(0, 234, 21),
        mk_rgb!(0, 231, 24),
        mk_rgb!(0, 229, 26),
        mk_rgb!(0, 226, 29),
        mk_rgb!(0, 223, 32),
        mk_rgb!(0, 221, 34),
        mk_rgb!(0, 218, 37),
        mk_rgb!(0, 215, 40),
        mk_rgb!(0, 212, 43),
        mk_rgb!(0, 210, 45),
        mk_rgb!(0, 207, 48),
        mk_rgb!(0, 204, 51),
        mk_rgb!(0, 202, 53),
        mk_rgb!(0, 199, 56),
        mk_rgb!(0, 196, 59),
        mk_rgb!(0, 194, 61),
        mk_rgb!(0, 191, 64),
        mk_rgb!(0, 188, 67),
        mk_rgb!(0, 186, 69),
        mk_rgb!(0, 183, 72),
        mk_rgb!(0, 180, 75),
        mk_rgb!(0, 178, 77),
        mk_rgb!(0, 175, 80),
        mk_rgb!(0, 172, 83),
        mk_rgb!(0, 171, 85),
        mk_rgb!(0, 166, 90),
        mk_rgb!(0, 161, 95),
        mk_rgb!(0, 155, 101),
        mk_rgb!(0, 150, 106),
        mk_rgb!(0, 145, 111),
        mk_rgb!(0, 139, 117),
        mk_rgb!(0, 134, 122),
        mk_rgb!(0, 129, 127),
        mk_rgb!(0, 123, 133),
        mk_rgb!(0, 118, 138),
        mk_rgb!(0, 113, 143),
        mk_rgb!(0, 107, 149),
        mk_rgb!(0, 102, 154),
        mk_rgb!(0, 97, 159),
        mk_rgb!(0, 91, 165),
        mk_rgb!(0, 86, 170),
        mk_rgb!(0, 81, 175),
        mk_rgb!(0, 75, 181),
        mk_rgb!(0, 70, 186),
        mk_rgb!(0, 65, 191),
        mk_rgb!(0, 59, 197),
        mk_rgb!(0, 54, 202),
        mk_rgb!(0, 49, 207),
        mk_rgb!(0, 43, 213),
        mk_rgb!(0, 38, 218),
        mk_rgb!(0, 33, 223),
        mk_rgb!(0, 27, 229),
        mk_rgb!(0, 22, 234),
        mk_rgb!(0, 17, 239),
        mk_rgb!(0, 11, 245),
        mk_rgb!(0, 6, 250),
        mk_rgb!(0, 0, 255),
        mk_rgb!(2, 0, 253),
        mk_rgb!(5, 0, 250),
        mk_rgb!(8, 0, 247),
        mk_rgb!(10, 0, 245),
        mk_rgb!(13, 0, 242),
        mk_rgb!(16, 0, 239),
        mk_rgb!(18, 0, 237),
        mk_rgb!(21, 0, 234),
        mk_rgb!(24, 0, 231),
        mk_rgb!(26, 0, 229),
        mk_rgb!(29, 0, 226),
        mk_rgb!(32, 0, 223),
        mk_rgb!(34, 0, 221),
        mk_rgb!(37, 0, 218),
        mk_rgb!(40, 0, 215),
        mk_rgb!(43, 0, 212),
        mk_rgb!(45, 0, 210),
        mk_rgb!(48, 0, 207),
        mk_rgb!(51, 0, 204),
        mk_rgb!(53, 0, 202),
        mk_rgb!(56, 0, 199),
        mk_rgb!(59, 0, 196),
        mk_rgb!(61, 0, 194),
        mk_rgb!(64, 0, 191),
        mk_rgb!(67, 0, 188),
        mk_rgb!(69, 0, 186),
        mk_rgb!(72, 0, 183),
        mk_rgb!(75, 0, 180),
        mk_rgb!(77, 0, 178),
        mk_rgb!(80, 0, 175),
        mk_rgb!(83, 0, 172),
        mk_rgb!(85, 0, 171),
        mk_rgb!(87, 0, 169),
        mk_rgb!(90, 0, 166),
        mk_rgb!(93, 0, 163),
        mk_rgb!(95, 0, 161),
        mk_rgb!(98, 0, 158),
        mk_rgb!(101, 0, 155),
        mk_rgb!(103, 0, 153),
        mk_rgb!(106, 0, 150),
        mk_rgb!(109, 0, 147),
        mk_rgb!(111, 0, 145),
        mk_rgb!(114, 0, 142),
        mk_rgb!(117, 0, 139),
        mk_rgb!(119, 0, 137),
        mk_rgb!(122, 0, 134),
        mk_rgb!(125, 0, 131),
        mk_rgb!(128, 0, 128),
        mk_rgb!(130, 0, 126),
        mk_rgb!(133, 0, 123),
        mk_rgb!(136, 0, 120),
        mk_rgb!(138, 0, 118),
        mk_rgb!(141, 0, 115),
        mk_rgb!(144, 0, 112),
        mk_rgb!(146, 0, 110),
        mk_rgb!(149, 0, 107),
        mk_rgb!(152, 0, 104),
        mk_rgb!(154, 0, 102),
        mk_rgb!(157, 0, 99),
        mk_rgb!(160, 0, 96),
        mk_rgb!(162, 0, 94),
        mk_rgb!(165, 0, 91),
        mk_rgb!(168, 0, 88),
        mk_rgb!(170, 0, 85),
        mk_rgb!(172, 0, 83),
        mk_rgb!(175, 0, 80),
        mk_rgb!(178, 0, 77),
        mk_rgb!(180, 0, 75),
        mk_rgb!(183, 0, 72),
        mk_rgb!(186, 0, 69),
        mk_rgb!(188, 0, 67),
        mk_rgb!(191, 0, 64),
        mk_rgb!(194, 0, 61),
        mk_rgb!(196, 0, 59),
        mk_rgb!(199, 0, 56),
        mk_rgb!(202, 0, 53),
        mk_rgb!(204, 0, 51),
        mk_rgb!(207, 0, 48),
        mk_rgb!(210, 0, 45),
        mk_rgb!(213, 0, 42),
        mk_rgb!(215, 0, 40),
        mk_rgb!(218, 0, 37),
        mk_rgb!(221, 0, 34),
        mk_rgb!(223, 0, 32),
        mk_rgb!(226, 0, 29),
        mk_rgb!(229, 0, 26),
        mk_rgb!(231, 0, 24),
        mk_rgb!(234, 0, 21),
        mk_rgb!(237, 0, 18),
        mk_rgb!(239, 0, 16),
        mk_rgb!(242, 0, 13),
        mk_rgb!(245, 0, 10),
        mk_rgb!(247, 0, 8),
        mk_rgb!(250, 0, 5),
        mk_rgb!(253, 0, 2),
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
