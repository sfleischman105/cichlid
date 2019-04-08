#[cfg(feature = "no-std")]
use core::mem::transmute;
#[cfg(not(feature = "no-std"))]
use std::mem::transmute;

/// Three Dimension Linear Interpolation
pub struct ThreePointLerp {
    pub delta: [i16; 3],
    pub accum: [u16; 3],
}

impl ThreePointLerp {
    #[inline(always)]
    pub const fn new() -> Self {
        ThreePointLerp {
            delta: [0; 3],
            accum: [0; 3],
        }
    }

    #[inline(always)]
    pub fn set_lerp_from_diff(self, num: usize, start: u8, end: u8) -> Self {
        let distance: i16 = (i16::from(end).wrapping_sub(i16::from(start))).wrapping_shl(7);
        self.set_lerp_from_distance(num, start, distance)
    }

    #[inline(always)]
    pub fn set_lerp_from_distance(mut self, num: usize, start: u8, distance: i16) -> Self {
        assert!(num <= 2);
        self.delta[num] = distance;
        self.accum[num] = u16::from(start) << 8;
        self
    }

    #[inline(always)]
    pub fn modify_delta<F>(mut self, mut f: F) -> Self
    where
        for<'w> F: FnMut(i16) -> i16,
    {
        self.delta.iter_mut().for_each(|x| *x = f(*x));
        self
    }

    #[inline(always)]
    fn lerp(&self) -> (u8, u8, u8) {
        (
            (self.accum[0] >> 8) as u8,
            (self.accum[1] >> 8) as u8,
            (self.accum[2] >> 8) as u8,
        )
    }
}

impl Iterator for ThreePointLerp {
    type Item = (u8, u8, u8);

    #[inline]
    fn next(&mut self) -> Option<(u8, u8, u8)> {
        let tuple: (u8, u8, u8) = self.lerp();
        self.accum
            .iter_mut()
            .zip(self.delta.iter())
            .for_each(|(a, d)| *a = a.wrapping_add(unsafe { transmute::<i16, u16>(*d) }));
        Some(tuple)
    }
}
