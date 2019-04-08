//! Collection of math functions for fast trigonometry.
//!
//! Credit for most of these functions goes to the authors of the FastLED library.

#[cfg(feature = "no-std")]
use core::mem::transmute;
#[cfg(not(feature = "no-std"))]
use std::mem::transmute;

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
pub fn cos16(theta: u16) -> i16 {
    sin16(theta.wrapping_add(16384))
}

/// Returns the sine of a single byte integer.
pub fn sin8(theta: u8) -> u8 {
    static B_M16_INTERLEAVE: [u8; 8] = [0, 49, 49, 41, 90, 27, 117, 10];

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

/// Returns the cosine of a single byte integer.
pub fn cos8(theta: u8) -> u8 {
    sin8(theta.wrapping_add(64))
}
