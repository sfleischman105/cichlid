//! Collection of math functions for scaling, dimming, and brightening numbers.
//!
//! Credit for most of these functions goes to the authoers of the FastLED library.

#![allow(clippy::cast_lossless)]

/// Scales one byte (`i`) by a second one (`scale`), which is treated as the numerator
/// of a fraction whose denominator is `256`.
///
/// In other words, it computes `i * (scale / 256)`
///
/// # Example
///
/// ```
/// use cichlid::math::scale::scale8;
///
/// assert_eq!(scale8(100, 255), 100); // 100 * 1.0
/// assert_eq!(scale8(100, 0), 0); // 100 * 0.0
/// assert_eq!(scale8(100, 255 / 2), 50); // 100 * 0.5
/// ```
#[inline(always)]
pub const fn scale8(i: u8, scale: u8) -> u8 {
    (((i as u16) * (1u16 + scale as u16)) >> 8) as u8
}

/// The "video" version of scale8.
///
/// This version guarantees that the output will be only be zero if one
/// or both of the inputs are zero.  If both inputs are non-zero, the output is guaranteed
/// to be non-zero.
///
/// This makes for better 'video'/LED dimming, at the cost of several additional cycles.
///
/// # Example
///
/// ```
/// use cichlid::math::scale::{scale8_video, scale8};
///
/// assert_eq!(scale8_video(100, 255), scale8(100, 255)); // same as scale8...
/// assert_ne!(scale8_video(1, 1),  scale8(1, 1));  // Except scale8() == 0
/// ```
#[inline]
pub const fn scale8_video(i: u8, scale: u8) -> u8 {
    let x: u8 = (((i as u16) * (scale as u16)) >> 8) as u8;
    let correction_int: u8 = (i != 0) as u8;
    let correction_scale: u8 = (scale != 0) as u8;
    let correction: u8 = correction_int & correction_scale;
    x + correction as u8
}

/// In place version of `scale8`.
///
/// # Example
///
/// ```
/// use cichlid::math::scale::nscale8;
///
/// let mut n: u8 = 100;
/// let frac: u8 = 255 / 2;
/// nscale8(&mut n, frac);
/// assert_eq!(n, 50); // same as scale8, but in place
/// ```
#[inline(always)]
pub fn nscale8(int: &mut u8, scale: u8) {
    *int = scale8(*int, scale);
}

/// In place version of `scale8`, but operating on two bytes with the same scale.
#[inline(always)]
pub fn nscale8x2(int_1: &mut u8, int_2: &mut u8, scale: u8) {
    let scaler: u16 = 1u16 + u16::from(scale);
    *int_1 = (((*int_1 as u16) * scaler) >> 8) as u8;
    *int_2 = (((*int_2 as u16) * scaler) >> 8) as u8;
}

/// In place version of `scale8`, but operating on three bytes with the same scale.
#[inline(always)]
pub fn nscale8x3(int_1: &mut u8, int_2: &mut u8, int_3: &mut u8, scale: u8) {
    let scaler: u16 = 1u16 + u16::from(scale);
    *int_1 = (((*int_1 as u16) * scaler) >> 8) as u8;
    *int_2 = (((*int_2 as u16) * scaler) >> 8) as u8;
    *int_3 = (((*int_3 as u16) * scaler) >> 8) as u8;
}

/// In place version of `scale8`, but operating on four bytes with the same scale.
#[inline(always)]
pub fn nscale8x4(int_1: &mut u8, int_2: &mut u8, int_3: &mut u8, int_4: &mut u8, scale: u8) {
    let scaler: u16 = 1u16 + u16::from(scale);
    *int_1 = (((*int_1 as u16) * scaler) >> 8) as u8;
    *int_2 = (((*int_2 as u16) * scaler) >> 8) as u8;
    *int_3 = (((*int_3 as u16) * scaler) >> 8) as u8;
    *int_4 = (((*int_4 as u16) * scaler) >> 8) as u8;
}

/// Dims a byte.
///
/// The eye does not respond in a linear way to light. High speed PWM'd LEDs at 50% duty cycle
/// appear far brighter then the 'half as bright' you might expect.
///
/// If you want your midpoint brightness level (128) to appear half as bright as 'full' brightness
/// (255), you have to apply a dimming function.
///
/// # Example
///
/// ```
/// use cichlid::math::scale::dim8_raw;
///
/// let full_brightness: u8 = 255;
/// assert_eq!(255, dim8_raw(full_brightness));
///
/// let half_brightness: u8 = full_brightness / 2;
/// assert_eq!(63, dim8_raw(half_brightness));
/// ```
#[inline(always)]
pub const fn dim8_raw(x: u8) -> u8 {
    scale8(x, x)
}

/// Dims a byte in video mode.
///
/// This is the same as `dim8_raw`, but the output of this function will only be zero if the
/// parameter byte is zero.
///
/// # Example
///
/// ```
/// use cichlid::math::scale::{dim8_raw,dim8_video};
///
/// assert_eq!(dim8_raw(255), dim8_video(255));
/// assert_ne!(dim8_raw(30), dim8_video(30));
/// ```
#[inline(always)]
pub const fn dim8_video(x: u8) -> u8 {
    scale8_video(x, x)
}

/// Dims a byte in linearly.
///
/// This is the same as `dim8_raw`, but when `x < 128`, the value is simply halved. The output
/// will only be zero if the input is zero.
#[inline]
pub fn dim8_lin(mut x: u8) -> u8 {
    if (x & 0x80) != 0 {
        x = scale8(x, x);
    } else {
        x += 1;
        x /= 2;
    }
    x
}

/// Inverse of the `dim8_raw` function, brightens a value.
#[inline(always)]
pub const fn brighten8_raw(x: u8) -> u8 {
    let ix = 255 - x;
    255 - dim8_raw(ix)
}

/// Inverse of the `dim8_video` function, brightens a value.
#[inline(always)]
pub const fn brighten8_video(x: u8) -> u8 {
    let ix = 255 - x;
    255 - dim8_video(ix)
}

/// Linear version of the `brighten8_raw`, that halves for values < 128.
///
/// It is also the inverse of `dim8_lin`.
#[inline]
pub fn brighten8_lin(x: u8) -> u8 {
    let ix = 255 - x;
    255 - dim8_lin(ix)
}

#[inline(always)]
pub const fn scale16(i: u16, scale: u16) -> u16 {
    (((i as u32) * (1u32 + scale as u32)) >> 16) as u16
}

#[inline(always)]
pub const fn scale16by8(i: u16, scale: u8) -> u16 {
    ((i as u32 * (1u32 + scale as u32)) >> 8) as u16
}
