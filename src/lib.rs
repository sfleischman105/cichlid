#![cfg_attr(feature="no-std", no_std)]
#![feature(asm)]
#![feature(const_panic)]

pub mod scale;
pub mod trig;
pub mod color_codes;
pub mod rgb;
pub mod hsv;

pub use crate::rgb::ColorRGB;
pub use crate::hsv::HSV;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
