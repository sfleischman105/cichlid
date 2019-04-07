use crate::HSV;

impl<'a, T, H: 'a> super::FillRainbow<u8> for T
    where
        T: IntoIterator<Item=&'a mut H>,
        H: From<HSV> {
    fn fill_rainbow_with_sat_val(self, start_hue: u8, hue_delta: u8, sat: u8, val: u8) {
        let mut hue: u8 = start_hue;
        let mut hue_accm = || {
            let old = hue;
            hue = hue.wrapping_add(hue_delta);
            old
        };
        self.into_iter()
            .map(|p| (p, hue_accm()))
            .for_each(|(i, h)| *i = H::from(HSV::new(h, sat, val)));
    }
}


impl<'a, T, H: 'a> super::FillRainbow<u16> for T
    where
    T: IntoIterator<Item=&'a mut H>,
    H: From<HSV> {
    fn fill_rainbow_with_sat_val(self, start_hue: u8, hue_delta: u16, sat: u8, val: u8) {
        let mut hue: u16 = (start_hue as u16) << 8;
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

impl<'a, T, H: 'a> super::FillRainbow<u32> for T
    where
        T: IntoIterator<Item=&'a mut H>,
        H: From<HSV> {
    fn fill_rainbow_with_sat_val(self, start_hue: u8, hue_delta: u32, sat: u8, val: u8) {
        let mut hue: u32 = (start_hue as u32) << 24;
        let mut hue_accm = || {
            let old = hue;
            hue = hue.wrapping_add(hue_delta);
            (old >> 24) as u8
        };
        self.into_iter()
            .map(|p| (p, hue_accm()))
            .for_each(|(i, h)| *i = H::from(HSV::new(h, sat, val)));
    }
}

//TODO: Don't I need to add an ability for direciton?
impl<'a, T, H: 'a> super::FillSingularRainbow for T
    where
        T: IntoIterator<Item=&'a mut H>,
        T::IntoIter : ExactSizeIterator,
        H: From<HSV> {
    fn fill_singular_rainbow(self, start_hue: u8) {
        let iter = self.into_iter();
        let len = iter.len();
        let hue_delta = (255u32 << 24) / (len as u32);
        let mut hue: u32 = (start_hue as u32) << 24;
        let mut hue_accm = || {
            let old = hue;
            hue = hue.wrapping_add(hue_delta);
            (old >> 24) as u8
        };
        iter.map(|p| (p, hue_accm()))
            .for_each(|(i, h)| *i = H::from(HSV::new(h, 255, 255)));
    }
}