//! Collection of math functions for scaling, dimming, and brightening numbers.
//!
//! These are the raw functions for both `u8` and `u16`. All of these methods
//! are implemented through the `Scaling` trait interface, see that for a
//! documentation of these functions.
//!
//! If `const fn's` are desired, use this module instead of the trait impls.

// Credit for most of these functions goes to the authoers of the FastLED library.

#![allow(clippy::cast_lossless)]


pub use crate::math::scale::scale_u8_impls::scale as scale8;
pub use crate::math::scale::scale_u8_impls::scale_video as scale8_video;
pub use crate::math::scale::scale_u8_impls::dim_raw as dim8_raw;
pub use crate::math::scale::scale_u8_impls::dim_video as dim8_video;
pub use crate::math::scale::scale_u8_impls::dim_lin as dim8_lin;
pub use crate::math::scale::scale_u8_impls::brighten_raw as brighten8_raw;
pub use crate::math::scale::scale_u8_impls::brighten_video as brighten8_video;
pub use crate::math::scale::scale_u8_impls::brighten_lin as brighten8_lin;
pub use crate::math::scale::scale_u8_impls::nscale as nscale8;
pub use crate::math::scale::scale_u8_impls::nscale_x2 as nscale8x2;
pub use crate::math::scale::scale_u8_impls::nscale_x3 as nscale8x3;
pub use crate::math::scale::scale_u8_impls::nscale_x4 as nscale8x4;

pub use crate::math::scale::scale_u16_impls::scale as scale16;
pub use crate::math::scale::scale_u16_impls::scale_video as scale16_video;
pub use crate::math::scale::scale_u16_impls::dim_raw as dim16_raw;
pub use crate::math::scale::scale_u16_impls::dim_video as dim16_video;
pub use crate::math::scale::scale_u16_impls::dim_lin as dim16_lin;
pub use crate::math::scale::scale_u16_impls::brighten_raw as brighten16_raw;
pub use crate::math::scale::scale_u16_impls::brighten_video as brighten16_video;
pub use crate::math::scale::scale_u16_impls::brighten_lin as brighten16_lin;
pub use crate::math::scale::scale_u16_impls::nscale as nscale16;
pub use crate::math::scale::scale_u16_impls::nscale_x2 as nscale16x2;
pub use crate::math::scale::scale_u16_impls::nscale_x3 as nscale16x3;
pub use crate::math::scale::scale_u16_impls::nscale_x4 as nscale16x4;


macro_rules! impl_brighten_ops {
    ($t:tt, $up:tt, $max:expr, $fname:ident, $dname:ident) => {
        #[inline(always)]
        pub const fn $fname(x: $t) -> $t {
            let ix = $max - x;
            $max - $dname(ix)
        }
    };
}

macro_rules! impl_nscale_ops {
    ($t:tt, $up:tt, $mscaler:expr, $($element:tt),*) => {
         let scaler: $up = 1 as $up + $up::from($mscaler);
         $( *$element = (((*$element as $up) * scaler) >> 8) as $t; )*
    };
}

macro_rules! impl_scale_ops {
    ($t:tt, $up:tt, $shift:expr, $max:expr) => (
        #[inline(always)]
        pub const fn scale(i: $t, scale: $t) -> $t {
            (((i as $up) * (1 as $up + scale as $up)) >> $shift) as $t
        }

        #[inline]
        pub const fn scale_video(i: $t, scale: $t) -> $t {
            let x: $t = (((i as $up) * (scale as $up)) >> $shift) as $t;
            let correction_int: $t = (i != 0) as $t;
            let correction_scale: $t = (scale != 0) as $t;
            let correction: $t = correction_int & correction_scale;
            x + correction as $t
        }

        #[inline(always)]
        pub const fn dim_raw(x: $t) -> $t {
            scale(x, x)
        }

        #[inline(always)]
        pub const fn dim_video(x: $t) -> $t {
            scale_video(x, x)
        }

        #[inline]
        pub const fn dim_lin(mut x: $t) -> $t {
            const UPPER_BITS: $t = (1 << ($shift - 1));
            let use_lin = (x & UPPER_BITS) != 0;
            let scale_x_reg = (use_lin as $t) * scale(x, x);
            let scale_x_lin = (!use_lin as $t) * (x.wrapping_add(1) / 2);
            // This is just a hack to be able to use const fns.
            scale_x_reg.wrapping_add(scale_x_lin)
        }

        impl_brighten_ops!($t, $up, $max, brighten_raw, dim_raw);
        impl_brighten_ops!($t, $up, $max, brighten_video, dim_video);
        impl_brighten_ops!($t, $up, $max, brighten_lin, dim_lin);

        #[inline(always)]
        pub fn nscale(int: &mut $t, scaler: $t) {
            *int = scale(*int, scaler);
        }

        #[inline(always)]
        pub fn nscale_x2(int_1: &mut $t, int_2: &mut $t, scaler: $t) {
            impl_nscale_ops!($t, $up, scaler, int_1, int_2);
        }

        #[inline]
        pub fn nscale_x3(int_1: &mut $t, int_2: &mut $t, int_3: &mut $t, scaler: $t) {
            impl_nscale_ops!($t, $up, scaler, int_1, int_2, int_3);
        }

        #[inline]
        pub fn nscale_x4(int_1: &mut $t, int_2: &mut $t, int_3: &mut $t, int_4: &mut $t, scaler: $t) {
            impl_nscale_ops!($t, $up, scaler, int_1, int_2, int_3, int_4);
        }
    )
}

pub mod scale_u8_impls {
    //! Scaling functions for `u8`s.
    use super::*;
    impl_scale_ops!(u8, u16, 8, 255);
}


pub mod scale_u16_impls {
    //! Scaling functions for `u16`s.
    use super::*;
    impl_scale_ops!(u16, u32, 16, 65535);
}

impl super::Scaling for u8 {
    #[inline(always)]
    fn scale(self, other: u8) -> u8 {
        crate::math::scale::scale_u8_impls::scale(self, other)
    }

    #[inline(always)]
    fn scale_video(self, other: u8) -> u8 {
        crate::math::scale::scale_u8_impls::scale_video(self, other)
    }

    #[inline(always)]
    fn dim_raw(self) -> u8 {
        crate::math::scale::scale_u8_impls::dim_raw(self)
    }

    #[inline(always)]
    fn dim_video(self) -> u8{
        crate::math::scale::scale_u8_impls::dim_video(self)
    }

    #[inline(always)]
    fn dim_lin(self) -> u8{
        crate::math::scale::scale_u8_impls::dim_lin(self)
    }

    #[inline(always)]
    fn brighten(self) -> u8 {
        crate::math::scale::scale_u8_impls::brighten_raw(self)
    }

    #[inline(always)]
    fn brighten_video(self) -> u8 {
        crate::math::scale::scale_u8_impls::brighten_video(self)
    }

    #[inline(always)]
    fn brighten_lin(self) -> u8 {
        crate::math::scale::scale_u8_impls::brighten_lin(self)
    }
}


impl super::Scaling for u16 {
    #[inline(always)]
    fn scale(self, other: u16) -> u16 {
        crate::math::scale::scale_u16_impls::scale(self, other)
    }

    #[inline(always)]
    fn scale_video(self, other: u16) -> u16 {
        crate::math::scale::scale_u16_impls::scale_video(self, other)
    }

    #[inline(always)]
    fn dim_raw(self) -> u16 {
        crate::math::scale::scale_u16_impls::dim_raw(self)
    }

    #[inline(always)]
    fn dim_video(self) -> u16{
        crate::math::scale::scale_u16_impls::dim_video(self)
    }

    #[inline(always)]
    fn dim_lin(self) -> u16{
        crate::math::scale::scale_u16_impls::dim_lin(self)
    }

    #[inline(always)]
    fn brighten(self) -> u16 {
        crate::math::scale::scale_u16_impls::brighten_raw(self)
    }

    #[inline(always)]
    fn brighten_video(self) -> u16 {
        crate::math::scale::scale_u16_impls::brighten_video(self)
    }

    #[inline(always)]
    fn brighten_lin(self) -> u16 {
        crate::math::scale::scale_u16_impls::brighten_lin(self)
    }
}