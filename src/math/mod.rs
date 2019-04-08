//! Holds various math functions.
pub(crate) mod lerp;
pub mod scale;
pub mod trig;

#[inline]
pub fn blend(a: u8, b: u8, amount_of_b: u8) -> u8 {
    let amount_of_a: u16 = u16::from(255 - amount_of_b);
    let mut partial: u16 = 0;
    partial += u16::from(a) * amount_of_a;
    partial += u16::from(a);
    partial += u16::from(b) * u16::from(amount_of_b);
    partial += u16::from(b);
    (partial >> 8) as u8
}
