#[cfg(feature="no-std")]
use core::mem::transmute;
#[cfg(not(feature="no-std"))]
use std::mem::transmute;

#[cfg(feature="no-std")]
use core::cmp::{Ord, Ordering, PartialOrd};
#[cfg(not(feature="no-std"))]
use std::cmp::{Ord, Ordering, PartialOrd};

#[cfg(feature="no-std")]
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Div, DivAssign, Index, IndexMut, Mul,
    MulAssign, Neg, Not, Rem, ShrAssign, Sub, SubAssign,
};
#[cfg(not(feature="no-std"))]
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Div, DivAssign, Index, IndexMut, Mul,
    MulAssign, Neg, Not, Rem, ShrAssign, Sub, SubAssign,
};

use crate::scale::*;
use crate::HSV;

pub trait RGBOrder {
    const FIRST: usize;
    const SECOND: usize;
    const THIRD: usize;
}

macro_rules! impl_order {
    ($t:tt, $o1:expr, $o2:expr, $o3:expr) => {
        pub struct $t;
        impl RGBOrder for $t {
            const FIRST: usize = $o1;
            const SECOND: usize = $o2;
            const THIRD: usize = $o3;
        }
    };
}

impl_order!(OrderingRGB, 0, 1, 2);
impl_order!(OrderingRBG, 0, 2, 1);
impl_order!(OrderingGRB, 1, 0, 2);
impl_order!(OrderingBRG, 1, 2, 0);
impl_order!(OrderingGBR, 2, 0, 1);
impl_order!(OrderingBGR, 2, 1, 2);

#[repr(packed)]
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct ColorRGB {
    pub r: u8,
    pub b: u8,
    pub g: u8,
}

impl ColorRGB {
    #[inline(always)]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        ColorRGB { r, g, b }
    }

    #[inline(always)]
    pub const fn from_color_code(code: u32) -> Self {
        ColorRGB {
            r: (code >> 16) as u8 & 0xFF,
            g: (code >> 8) as u8 & 0xFF,
            b: (code >> 0) as u8 & 0xFF,
        }
    }

    pub fn clear(&mut self) {
        self.modify_all(|_| 0);
    }

    #[inline(always)]
    pub fn r(&self) -> u8 {
        self.r
    }
    #[inline(always)]
    pub fn g(&self) -> u8 {
        self.g
    }
    #[inline(always)]
    pub fn b(&self) -> u8 {
        self.b
    }
    #[inline(always)]
    pub fn red(&self) -> u8 {
        self.r
    }
    #[inline(always)]
    pub fn green(&self) -> u8 {
        self.g
    }
    #[inline(always)]
    pub fn blue(&self) -> u8 {
        self.b
    }

    #[inline(always)]
    pub fn set_red(&mut self, r: u8) {
        self.r = r;
    }
    #[inline(always)]
    pub fn set_green(&mut self, g: u8) {
        self.g = g;
    }
    #[inline(always)]
    pub fn set_blue(&mut self, b: u8) {
        self.b = b;
    }

    #[inline]
    pub fn modify_red<F>(&mut self, mut f: F)
        where
                for<'w> F: FnMut(u8) -> u8,
    {
        self.r = f(self.r);
    }

    #[inline]
    pub fn modify_green<F>(&mut self, mut f: F)
        where
                for<'w> F: FnMut(u8) -> u8,
    {
        self.g = f(self.g);
    }

    #[inline]
    pub fn modify_blue<F>(&mut self, mut f: F)
        where
                for<'w> F: FnMut(u8) -> u8,
    {
        self.b = f(self.b);
    }

    #[inline]
    pub fn modify_all<F>(&mut self, mut f: F)
        where
                for<'w> F: FnMut(u8) -> u8,
    {
        self.r = f(self.r);
        self.g = f(self.g);
        self.b = f(self.b);
    }

    #[inline]
    pub fn luma(&self) -> u8 {
        let mut luma: u8 = 0;
        luma += scale8(self.r, 54);
        luma += scale8(self.g, 183);
        luma += scale8(self.b, 18);
        luma
    }

    #[inline]
    pub fn avg_light(&self) -> u8 {
        let mut luma: u8 = 0;
        luma += scale8(self.r, 85);
        luma += scale8(self.g, 85);
        luma += scale8(self.b, 85);
        luma
    }

    #[inline]
    pub fn maximize_brightness(&mut self) {
        let maxi: u8 = self.r.max(self.g.max(self.b));
        let b_factor: u16 = (maxi as u16 * 256) / maxi as u16;
        self.modify_all(|c| ((b_factor * c as u16) / 256) as u8);
    }
}

#[allow(non_upper_case_globals)]
impl ColorRGB {
    pub const AliceBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::AliceBlue);
    pub const Amethyst: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Amethyst);
    pub const AntiqueWhite: ColorRGB = ColorRGB::from_color_code(crate::color_codes::AntiqueWhite);
    pub const Aqua: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Aqua);
    pub const Aquamarine: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Aquamarine);
    pub const Azure: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Azure);
    pub const Beige: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Beige);
    pub const Bisque: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Bisque);
    pub const Black: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Black);
    pub const BlanchedAlmond: ColorRGB = ColorRGB::from_color_code(crate::color_codes::BlanchedAlmond);
    pub const Blue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Blue);
    pub const BlueViolet: ColorRGB = ColorRGB::from_color_code(crate::color_codes::BlueViolet);
    pub const Brown: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Brown);
    pub const BurlyWood: ColorRGB = ColorRGB::from_color_code(crate::color_codes::BurlyWood);
    pub const CadetBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::CadetBlue);
    pub const Chartreuse: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Chartreuse);
    pub const Chocolate: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Chocolate);
    pub const Coral: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Coral);
    pub const CornflowerBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::CornflowerBlue);
    pub const Cornsilk: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Cornsilk);
    pub const Crimson: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Crimson);
    pub const Cyan: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Cyan);
    pub const DarkBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkBlue);
    pub const DarkCyan: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkCyan);
    pub const DarkGoldenrod: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkGoldenrod);
    pub const DarkGray: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkGray);
    pub const DarkGrey: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkGrey);
    pub const DarkGreen: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkGreen);
    pub const DarkKhaki: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkKhaki);
    pub const DarkMagenta: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkMagenta);
    pub const DarkOliveGreen: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkOliveGreen);
    pub const DarkOrange: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkOrange);
    pub const DarkOrchid: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkOrchid);
    pub const DarkRed: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkRed);
    pub const DarkSalmon: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkSalmon);
    pub const DarkSeaGreen: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkSeaGreen);
    pub const DarkSlateBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkSlateBlue);
    pub const DarkSlateGray: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkSlateGray);
    pub const DarkSlateGrey: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkSlateGrey);
    pub const DarkTurquoise: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkTurquoise);
    pub const DarkViolet: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DarkViolet);
    pub const DeepPink: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DeepPink);
    pub const DeepSkyBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DeepSkyBlue);
    pub const DimGray: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DimGray);
    pub const DimGrey: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DimGrey);
    pub const DodgerBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::DodgerBlue);
    pub const FireBrick: ColorRGB = ColorRGB::from_color_code(crate::color_codes::FireBrick);
    pub const FloralWhite: ColorRGB = ColorRGB::from_color_code(crate::color_codes::FloralWhite);
    pub const ForestGreen: ColorRGB = ColorRGB::from_color_code(crate::color_codes::ForestGreen);
    pub const Fuchsia: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Fuchsia);
    pub const Gainsboro: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Gainsboro);
    pub const GhostWhite: ColorRGB = ColorRGB::from_color_code(crate::color_codes::GhostWhite);
    pub const Gold: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Gold);
    pub const Goldenrod: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Goldenrod);
    pub const Gray: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Gray);
    pub const Grey: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Grey);
    pub const Green: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Green);
    pub const GreenYellow: ColorRGB = ColorRGB::from_color_code(crate::color_codes::GreenYellow);
    pub const Honeydew: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Honeydew);
    pub const HotPink: ColorRGB = ColorRGB::from_color_code(crate::color_codes::HotPink);
    pub const IndianRed: ColorRGB = ColorRGB::from_color_code(crate::color_codes::IndianRed);
    pub const Indigo: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Indigo);
    pub const Ivory: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Ivory);
    pub const Khaki: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Khaki);
    pub const Lavender: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Lavender);
    pub const LavenderBlush: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LavenderBlush);
    pub const LawnGreen: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LawnGreen);
    pub const LemonChiffon: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LemonChiffon);
    pub const LightBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LightBlue);
    pub const LightCoral: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LightCoral);
    pub const LightCyan: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LightCyan);
    pub const LightGoldenrodYellow: ColorRGB =
        ColorRGB::from_color_code(crate::color_codes::LightGoldenrodYellow);
    pub const LightGreen: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LightGreen);
    pub const LightGrey: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LightGrey);
    pub const LightPink: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LightPink);
    pub const LightSalmon: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LightSalmon);
    pub const LightSeaGreen: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LightSeaGreen);
    pub const LightSkyBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LightSkyBlue);
    pub const LightSlateGray: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LightSlateGray);
    pub const LightSlateGrey: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LightSlateGrey);
    pub const LightSteelBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LightSteelBlue);
    pub const LightYellow: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LightYellow);
    pub const Lime: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Lime);
    pub const LimeGreen: ColorRGB = ColorRGB::from_color_code(crate::color_codes::LimeGreen);
    pub const Linen: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Linen);
    pub const Magenta: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Magenta);
    pub const Maroon: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Maroon);
    pub const MediumAquamarine: ColorRGB =
        ColorRGB::from_color_code(crate::color_codes::MediumAquamarine);
    pub const MediumBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::MediumBlue);
    pub const MediumOrchid: ColorRGB = ColorRGB::from_color_code(crate::color_codes::MediumOrchid);
    pub const MediumPurple: ColorRGB = ColorRGB::from_color_code(crate::color_codes::MediumPurple);
    pub const MediumSeaGreen: ColorRGB = ColorRGB::from_color_code(crate::color_codes::MediumSeaGreen);
    pub const MediumSlateBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::MediumSlateBlue);
    pub const MediumSpringGreen: ColorRGB =
        ColorRGB::from_color_code(crate::color_codes::MediumSpringGreen);
    pub const MediumTurquoise: ColorRGB = ColorRGB::from_color_code(crate::color_codes::MediumTurquoise);
    pub const MediumVioletRed: ColorRGB = ColorRGB::from_color_code(crate::color_codes::MediumVioletRed);
    pub const MidnightBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::MidnightBlue);
    pub const MintCream: ColorRGB = ColorRGB::from_color_code(crate::color_codes::MintCream);
    pub const MistyRose: ColorRGB = ColorRGB::from_color_code(crate::color_codes::MistyRose);
    pub const Moccasin: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Moccasin);
    pub const NavajoWhite: ColorRGB = ColorRGB::from_color_code(crate::color_codes::NavajoWhite);
    pub const Navy: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Navy);
    pub const OldLace: ColorRGB = ColorRGB::from_color_code(crate::color_codes::OldLace);
    pub const Olive: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Olive);
    pub const OliveDrab: ColorRGB = ColorRGB::from_color_code(crate::color_codes::OliveDrab);
    pub const Orange: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Orange);
    pub const OrangeRed: ColorRGB = ColorRGB::from_color_code(crate::color_codes::OrangeRed);
    pub const Orchid: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Orchid);
    pub const PaleGoldenrod: ColorRGB = ColorRGB::from_color_code(crate::color_codes::PaleGoldenrod);
    pub const PaleGreen: ColorRGB = ColorRGB::from_color_code(crate::color_codes::PaleGreen);
    pub const PaleTurquoise: ColorRGB = ColorRGB::from_color_code(crate::color_codes::PaleTurquoise);
    pub const PaleVioletRed: ColorRGB = ColorRGB::from_color_code(crate::color_codes::PaleVioletRed);
    pub const PapayaWhip: ColorRGB = ColorRGB::from_color_code(crate::color_codes::PapayaWhip);
    pub const PeachPuff: ColorRGB = ColorRGB::from_color_code(crate::color_codes::PeachPuff);
    pub const Peru: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Peru);
    pub const Pink: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Pink);
    pub const Plaid: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Plaid);
    pub const Plum: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Plum);
    pub const PowderBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::PowderBlue);
    pub const Purple: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Purple);
    pub const Red: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Red);
    pub const RosyBrown: ColorRGB = ColorRGB::from_color_code(crate::color_codes::RosyBrown);
    pub const RoyalBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::RoyalBlue);
    pub const SaddleBrown: ColorRGB = ColorRGB::from_color_code(crate::color_codes::SaddleBrown);
    pub const Salmon: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Salmon);
    pub const SandyBrown: ColorRGB = ColorRGB::from_color_code(crate::color_codes::SandyBrown);
    pub const SeaGreen: ColorRGB = ColorRGB::from_color_code(crate::color_codes::SeaGreen);
    pub const Seashell: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Seashell);
    pub const Sienna: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Sienna);
    pub const Silver: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Silver);
    pub const SkyBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::SkyBlue);
    pub const SlateBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::SlateBlue);
    pub const SlateGray: ColorRGB = ColorRGB::from_color_code(crate::color_codes::SlateGray);
    pub const SlateGrey: ColorRGB = ColorRGB::from_color_code(crate::color_codes::SlateGrey);
    pub const Snow: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Snow);
    pub const SpringGreen: ColorRGB = ColorRGB::from_color_code(crate::color_codes::SpringGreen);
    pub const SteelBlue: ColorRGB = ColorRGB::from_color_code(crate::color_codes::SteelBlue);
    pub const Tan: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Tan);
    pub const Teal: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Teal);
    pub const Thistle: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Thistle);
    pub const Tomato: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Tomato);
    pub const Turquoise: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Turquoise);
    pub const Violet: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Violet);
    pub const Wheat: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Wheat);
    pub const White: ColorRGB = ColorRGB::from_color_code(crate::color_codes::White);
    pub const WhiteSmoke: ColorRGB = ColorRGB::from_color_code(crate::color_codes::WhiteSmoke);
    pub const Yellow: ColorRGB = ColorRGB::from_color_code(crate::color_codes::Yellow);
    pub const YellowGreen: ColorRGB = ColorRGB::from_color_code(crate::color_codes::YellowGreen);
}

impl From<(u8, u8, u8)> for ColorRGB {
    #[inline(always)]
    fn from(other: (u8, u8, u8)) -> Self {
        unsafe { transmute(other) }
    }
}

impl From<[u8; 3]> for ColorRGB {
    #[inline(always)]
    fn from(other: [u8; 3]) -> Self {
        unsafe { transmute(other) }
    }
}

impl From<HSV> for ColorRGB {
    fn from(hsv: HSV) -> Self {
        hsv.to_rgb_rainbow()
    }
}

impl Index<usize> for ColorRGB {
    type Output = u8;
    #[inline(always)]
    fn index(&self, idx: usize) -> &u8 {
        unsafe {
            let arr: &[u8; 3] = transmute(self);
            &arr[idx]
        }
    }
}

impl IndexMut<usize> for ColorRGB {
    #[inline(always)]
    fn index_mut(&mut self, idx: usize) -> &mut u8 {
        unsafe {
            let arr: &mut [u8; 3] = transmute(self);
            &mut arr[idx]
        }
    }
}

impl AddAssign for ColorRGB {
    #[inline(always)]
    fn add_assign(&mut self, rhs: ColorRGB) {
        *self = ColorRGB {
            r: self.r.saturating_add(rhs.r),
            g: self.g.saturating_add(rhs.g),
            b: self.b.saturating_add(rhs.b),
        };
    }
}

// Add a constant to each channel
impl AddAssign<u8> for ColorRGB {
    #[inline(always)]
    fn add_assign(&mut self, rhs: u8) {
        self.modify_all(|c| c.saturating_add(rhs))
    }
}

impl SubAssign for ColorRGB {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: ColorRGB) {
        *self = ColorRGB {
            r: self.r.saturating_sub(rhs.r),
            g: self.g.saturating_sub(rhs.g),
            b: self.b.saturating_sub(rhs.b),
        };
    }
}

impl SubAssign<u8> for ColorRGB {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: u8) {
        self.modify_all(|c| c.saturating_sub(rhs))
    }
}

impl DivAssign<u8> for ColorRGB {
    #[inline(always)]
    fn div_assign(&mut self, rhs: u8) {
        self.modify_all(|c| c / rhs)
    }
}

impl MulAssign<u8> for ColorRGB {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: u8) {
        self.modify_all(|c| c.saturating_mul(rhs))
    }
}

impl ShrAssign<u8> for ColorRGB {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u8) {
        self.modify_all(|c| c >> rhs)
    }
}

impl BitOrAssign for ColorRGB {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: ColorRGB) {
        *self = ColorRGB {
            r: self.r.max(rhs.r),
            g: self.g.max(rhs.g),
            b: self.b.max(rhs.b),
        };
    }
}

impl BitOrAssign<u8> for ColorRGB {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: u8) {
        self.modify_all(|c| c.max(rhs))
    }
}

impl BitAndAssign for ColorRGB {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: ColorRGB) {
        *self = ColorRGB {
            r: self.r.min(rhs.r),
            g: self.g.min(rhs.g),
            b: self.b.min(rhs.b),
        };
    }
}

impl BitAndAssign<u8> for ColorRGB {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: u8) {
        self.modify_all(|c| c.min(rhs))
    }
}

impl Neg for ColorRGB {
    type Output = ColorRGB;

    #[inline(always)]
    fn neg(self) -> ColorRGB {
        let mut cln: ColorRGB = self.clone();
        cln.modify_all(|c| 255 - c);
        cln
    }
}

impl Not for ColorRGB {
    type Output = bool;
    #[inline(always)]
    fn not(self) -> bool {
        self.r != 0 || self.g != 0 || self.b != 0
    }
}

impl PartialOrd for ColorRGB {
    #[inline]
    fn partial_cmp(&self, other: &ColorRGB) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ColorRGB {
    #[inline]
    fn cmp(&self, rhs: &ColorRGB) -> Ordering {
        let rhs_t: u16 = rhs.r as u16 + rhs.b as u16 + rhs.g as u16;
        let lhs_t: u16 = self.r as u16 + self.b as u16 + self.g as u16;
        lhs_t.cmp(&rhs_t)
    }
}

impl Add for ColorRGB {
    type Output = ColorRGB;
    #[inline(always)]
    fn add(self, other: ColorRGB) -> ColorRGB {
        let mut cln: ColorRGB = self.clone();
        cln += other;
        cln
    }
}

impl Sub for ColorRGB {
    type Output = ColorRGB;
    #[inline(always)]
    fn sub(self, other: ColorRGB) -> ColorRGB {
        let mut cln: ColorRGB = self.clone();
        cln -= other;
        cln
    }
}

impl Mul<u8> for ColorRGB {
    type Output = ColorRGB;
    #[inline(always)]
    fn mul(self, rhs: u8) -> ColorRGB {
        let mut cln: ColorRGB = self.clone();
        cln *= rhs;
        cln
    }
}

impl Div<u8> for ColorRGB {
    type Output = ColorRGB;
    #[inline(always)]
    fn div(self, rhs: u8) -> ColorRGB {
        let mut cln: ColorRGB = self.clone();
        cln /= rhs;
        cln
    }
}

impl BitAnd for ColorRGB {
    type Output = ColorRGB;
    #[inline(always)]
    fn bitand(self, other: ColorRGB) -> ColorRGB {
        let mut cln: ColorRGB = self.clone();
        cln &= other;
        cln
    }
}

impl BitOr for ColorRGB {
    type Output = ColorRGB;
    #[inline(always)]
    fn bitor(self, other: ColorRGB) -> ColorRGB {
        let mut cln: ColorRGB = self.clone();
        cln |= other;
        cln
    }
}

impl Rem<u8> for ColorRGB {
    type Output = ColorRGB;
    #[inline(always)]
    fn rem(self, rhs: u8) -> ColorRGB {
        ColorRGB {
            r: self.r % rhs,
            g: self.g % rhs,
            b: self.b % rhs,
        }
    }
}
