//! Collection of math functions for fast trigonometry.

// Credit for most of these functions goes to the authors of the FastLED library.

/// Returns the sine of a two byte integer.
pub fn sin16(theta: u16) -> i16 {
    static BASE: [u16; 8] = [0, 6393, 12539, 18204, 23170, 27245, 30273, 32137];
    static SLOPE: [u8; 8] = [49, 48, 44, 38, 31, 23, 14, 4];
    let mut offset = (theta & 0x3FFF) >> 3;
    if (theta & 0x4000) != 0 {
        offset = 2057 - offset;
    }

    let section: u8 = (offset / 256) as u8;
    let b: u16 = unsafe { *BASE.get_unchecked(section as usize) };
    let m: u16 = u16::from(unsafe { *SLOPE.get_unchecked(section as usize) });

    let secoffset8: u8 = (offset as u8) / 2;
    let mx: u16 = m * u16::from(secoffset8);
    let mut y: i16 = (mx + b) as i16;
    if (theta & 0x8000) != 0 {
        y = -y;
    }
    y
}

/// Returns the cosine of a two byte integer.
#[inline(always)]
pub fn cos16(theta: u16) -> i16 {
    sin16(theta.wrapping_add(16384))
}

/// Returns the sine of a single byte integer.
#[inline(always)]
pub fn sin8(theta: u8) -> u8 {
    trig_inner::sin8(theta)
}

/// Returns the cosine of a single byte integer.
#[inline(always)]
pub fn cos8(theta: u8) -> u8 {
    sin8(theta.wrapping_add(64))
}

#[cfg(feature = "low-mem")]
mod trig_inner {
    #[cfg(feature = "no-std")]
    use core::mem::transmute;
    #[cfg(not(feature = "no-std"))]
    use std::mem::transmute;

    static B_M16_INTERLEAVE: [u8; 8] = [0, 49, 49, 41, 90, 27, 117, 10];

    pub fn sin8(theta: u8) -> u8 {
        let mut offset: u8 = theta;
        if theta & 0x40 != 0 {
            offset = 255 - offset;
        }
        offset &= 0x3F;

        let mut offset_two: u8 = offset & 0x0F;
        if theta & 0x40 != 0 {
            offset_two += 1;
        }

        let section_one: u8 = offset >> 4;
        let section_two: u8 = section_one * 2;

        let b: u8 = unsafe { *B_M16_INTERLEAVE.get_unchecked(section_two as usize) };
        let m16: u8 = unsafe { *B_M16_INTERLEAVE.get_unchecked(section_two as usize + 1) };
        let mx: u8 = m16.wrapping_mul(offset_two) >> 4;
        let mut y: i8 = unsafe { transmute(mx + b) };

        if theta & 0x80 != 0 {
            y = -y;
        }

        let sin: u8 = unsafe { transmute(y) };
        sin.wrapping_add(128)
    }
}

#[cfg(not(feature = "low-mem"))]
mod trig_inner {
    #[cfg(not(feature = "no-std"))]
    use std::intrinsics::transmute;
    #[cfg(feature = "no-std")]
    use core::intrinsics::transmute;

    #[inline(always)]
    pub fn sin8(theta: u8) -> u8 {
        unsafe { *SIN8_TABLE.get_unchecked(theta as usize) }
    }

    // TODO: What is this?

    static SIN8_TABLE: [u8; 256] = [
        128, 131, 134, 137, 140, 143, 130, 133, 136, 139, 142, 129, 132, 135, 138, 141, 177, 179,
        182, 184, 187, 189, 192, 178, 181, 184, 186, 189, 191, 178, 180, 183, 218, 219, 221, 223,
        224, 226, 228, 229, 231, 233, 218, 220, 222, 223, 225, 227, 245, 245, 246, 246, 247, 248,
        248, 249, 250, 250, 251, 251, 252, 253, 253, 254, 255, 254, 253, 253, 252, 251, 251, 250,
        250, 249, 248, 248, 247, 246, 246, 245, 229, 227, 225, 223, 222, 220, 218, 233, 231, 229,
        228, 226, 224, 223, 221, 219, 186, 183, 180, 178, 191, 189, 186, 184, 181, 178, 192, 189,
        187, 184, 182, 179, 129, 141, 138, 135, 132, 129, 142, 139, 136, 133, 130, 143, 140, 137,
        134, 131, 128, 125, 122, 119, 116, 113, 126, 123, 120, 117, 114, 127, 124, 121, 118, 115,
         79,  77,  74,  72,  69,  67,  64,  78,  75,  72,  70,  67,  65,  78,  76,  73,  38,  37,
         35,  33,  32,  30,  28,  27,  25,  23,  38,  36,  34,  33,  31,  29,  11,  11,  10,  10,
          9,   8,   8,   7,   6,   6,   5,   5,   4,   3,   3,   2,   1,   2,   3,   3,   4,   5,
          5,   6,   6,   7,   8,   8,   9,  10,  10,  11,  27,  29,  31,  33,  34,  36,  38,  23,
         25,  27,  28,  30,  32,  33,  35,  37,  70,  73,  76,  78,  65,  67,  70,  72,  75,  78,
         64,  67,  69,  72,  74,  77, 127, 115, 118, 121, 124, 127, 114, 117, 120, 123, 126, 113,
        116, 119, 122, 125,
    ];
}

impl super::Trig<u8> for u8 {
    fn sin(self) -> u8 {
        sin8(self)
    }

    fn cos(self) -> u8 {
        cos8(self)
    }
}

impl super::Trig<i16> for u16 {
    fn sin(self) -> i16 {
        sin16(self)
    }

    fn cos(self) -> i16 {
        cos16(self)
    }
}

#[cfg(test)]
mod test {
    use crate::math::trig::{cos8, sin8};

    #[test]
    fn all_sin() {
        for x in 0..=255 {
            sin8(x);
        }
    }

    #[test]
    fn all_cos() {
        for x in 0..=255 {
            cos8(x);
        }
    }

}
