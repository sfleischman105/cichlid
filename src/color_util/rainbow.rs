//! Rainbow gradient impls.

use crate::HSV;


impl<'a, T, H: 'a> super::RainbowFill for T
where
    T: IntoIterator<Item = &'a mut H>,
    H: From<HSV>,
{
    fn rainbow_fill_with_sat_val(self, start_hue: u8, hue_delta: u16, sat: u8, val: u8) {
        let mut hue: u16 = u16::from(start_hue) << 8;
        let mut hue_accm = || {
            let old = hue;
            hue = hue.wrapping_add(hue_delta);
            (old >> 8) as u8
        };
        self.into_iter()
            .map(|p| (p, hue_accm()))
            .for_each(|(i, h)| *i = H::from(HSV::new(h, sat, val)));
    }
}


impl<'a, T, H: 'a> super::RainbowFillSingleCycle for T
where
    T: IntoIterator<Item = &'a mut H>,
    T::IntoIter: ExactSizeIterator,
    H: From<HSV>,
{
    fn rainbow_fill_single_cycle(self, start_hue: u8) {
        let iter = self.into_iter();
        let len = iter.len();
        let hue_delta = (255u32 << 24) / (len as u32);
        let mut hue: u32 = u32::from(start_hue) << 24;
        let mut hue_accm = || {
            let old = hue;
            hue = hue.wrapping_add(hue_delta);
            (old >> 24) as u8
        };
        iter.map(|p| (p, hue_accm()))
            .for_each(|(i, h)| *i = H::from(HSV::new(h, 255, 255)));
    }
}
