///  scale one byte by a second one, which is treated as
///  the numerator of a fraction whose denominator is 256
///  In other words, it computes i * (scale / 256)
///  4 clocks AVR with MUL, 2 clocks ARM
#[inline(always)]
pub fn scale8(int: u8, scale: u8) -> u8 {
    (((int as u16) * (1u16 + scale as u16)) >> 8) as u8
}

///  The "video" version of scale8 guarantees that the output will
///  be only be zero if one or both of the inputs are zero.  If both
///  inputs are non-zero, the output is guaranteed to be non-zero.
///  This makes for better 'video'/LED dimming, at the cost of
///  several additional cycles.
#[inline(always)]
pub fn scale8_video(int: u8, scale: u8) -> u8 {
    match scale8(int, scale) {
        0 => 1,
        x => x,
    }
}

pub fn scale16(_int: u16, _scale: u16) -> u16 {
    unimplemented!()
}

pub fn dim8_raw(_c: u8) -> u8 {
    unimplemented!()
}

pub fn dim8_video(_c: u8) -> u8 {
    unimplemented!()
}

pub fn dim8_lin(_c: u8) -> u8 {
    unimplemented!()
}

/// inverse of the dimming function, brighten a value
#[inline]
pub fn brighten8_raw(c: u8) -> u8 {
    let ic = 255 - c;
    255 - scale8(ic, ic)
}

/// inverse of the dimming function, brighten a value
#[inline]
pub fn brighten8_video(c: u8) -> u8 {
    let ic = 255 - c;
    255 - scale8_video(ic, ic)
}

/// inverse of the dimming function, brighten a value
pub fn brighten8_lin(_c: u8) -> u8 {
    unimplemented!()
}
