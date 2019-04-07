use crate::HSV;

pub trait FillRainbow {
    fn fill_rainbow(&mut self, start_hue: u8, hue_delta: u8);
}

pub trait FillRainbowStepU16 {
    fn fill_rainbow(&mut self, start_hue: u8, hue_delta: u16);
}

impl FillRainbow for FillRainbowStepU16 {
    fn fill_rainbow(&mut self, start_hue: u8, hue_delta: u8) {
        <Self as FillRainbowStepU16>::fill_rainbow(self, start_hue, (hue_delta as u16) << 8);
    }
}