//! Collection of math Traits and functions for manipulating integers.
//!
//! Some including functions:
//! - Scaling Functions (from one integer to another)
//! - In place and batch scaling (`nscale16x3` for example).
//! - Dimming and Brightening Functions
//! - Fast u8 and u16 trigonometric functions
//! - Other useful operations, such as blending integers.
//!
//! This module offers a couple different ways to access the m
//! These are the raw functions for both `u8` and `u16`. Most of these methods
//! are implemented through the [`Scaling`] trait interface, see that for better
//! documentation of these functions.
//!
//! If `const` functions are desired, use the re-exported functions rather than
//! the trait impls.
//!
//! [`Scaling`]: ./trait.ScalingInt.html

// Credit for most of these functions goes to the authoers of the FastLED library.


#![allow(clippy::cast_lossless)]

// TODO: SIMD this stuff https://doc.rust-lang.org/core/arch/arm/index.html
// https://doc.rust-lang.org/edition-guide/rust-2018/simd-for-faster-computing.html

pub(crate) mod ext;
pub(crate) mod lerp;
pub(crate) mod trig;

pub use math_u16_impls::blend as blend_u16;
pub use math_u16_impls::brighten_lin as brighten_u16_lin;
pub use math_u16_impls::brighten_raw as brighten_u16_raw;
pub use math_u16_impls::brighten_video as brighten_u16_video;
pub use math_u16_impls::dim_lin as dim_u16_lin;
pub use math_u16_impls::dim_raw as dim_u16_raw;
pub use math_u16_impls::dim_video as dim_u16_video;
pub use math_u16_impls::nscale as nscale_u16;
pub use math_u16_impls::nscale_x2 as nscale_u16x2;
pub use math_u16_impls::nscale_x3 as nscale_u16x3;
pub use math_u16_impls::nscale_x4 as nscale_u16x4;
pub use math_u16_impls::scale as scale_u16;
pub use math_u16_impls::scale_video as scale_u16_video;
pub use math_u8_impls::blend as blend_u8;
pub use math_u8_impls::brighten_lin as brighten_u8_lin;
pub use math_u8_impls::brighten_raw as brighten_u8_raw;
pub use math_u8_impls::brighten_video as brighten_u8_video;
pub use math_u8_impls::dim_lin as dim_u8_lin;
pub use math_u8_impls::dim_raw as dim_u8_raw;
pub use math_u8_impls::dim_video as dim_u8_video;
pub use math_u8_impls::nscale as nscale_u8;
pub use math_u8_impls::nscale_x2 as nscale_u8x2;
pub use math_u8_impls::nscale_x3 as nscale_u8x3;
pub use math_u8_impls::nscale_x4 as nscale_u8x4;
pub use math_u8_impls::scale as scale_u8;
pub use math_u8_impls::scale_video as scale_u8_video;

pub use trig::{sin_u8,cos_u8,sin_u16,cos_u16};

/// Basic trigonometric functions for integers.
pub trait Trig<Signed> {
    fn sin(self) -> Signed;

    fn cos(self) -> Signed;
}

/// Scaling, Dimming, Brightening, and other misc functions functions for integers
/// representing scalar components.
///
/// These functions are extremely useful for operating on integer color components,
/// such as the red/blue/green values seen in `RGB` color encoding.
///
/// # Notes on Fractional Components
///
/// These methods are used primarily for representing integers as fractions, rather than
/// whole numbers. They can also be treated as fractions, percents, or some otherwise
/// range-bounded scalar to a dimension. A more accurate way to represent this information
/// would be to use a `f32`/`f64` and clamping the result to a pre-defined range.
/// Integers are used as the math is significantly faster to compute, and floating values
/// aren't always available on all target platforms.
///
/// For example, a `u8` takes on the range `[0:255]`. The maximum value
/// of 255 doesn't represent the existence of 255 items, but rather being the maximum
/// possible scalar for a dimension. Respectively, the value of 0 is the minimum value
/// for a dimension.
///
/// As a by-product, these functions are saturating, hitting a ceiling at the maximum
/// possible value, and hitting a floor at the minimum possible value (usually 0,
/// except for `_video` functions).
///
/// # Terminology
///
/// - `_video`: The output is guaranteed to only be zero if at least one of the
///   inputs is zero.
/// - `_lin`: Used only in brightening and dimming functions. If the input is below
///   half of the maximum value, the value is brightened / dimmed linearly instead of
///   scaled.
///
pub trait ScalingInt {
    /// Scales self by a second one (`scale`), which is treated as the numerator
    /// of a fraction whose denominator is `Self::MAX`.
    ///
    /// In other words, it computes `i * (scale / Self::MAX)`
    ///
    /// # Example
    ///
    /// ```
    /// use cichlid::prelude::ScalingInt;
    ///
    /// assert_eq!(100u8.scale(255), 100); // 100 * 1.0
    /// assert_eq!(100u8.scale(0), 0); // 100 * 0.0
    /// assert_eq!(100u8.scale(255 / 2), 50); // 100 * 0.5
    /// ```
    fn scale(self, other: Self) -> Self;

    /// The "video" version of scale.
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
    /// use cichlid::prelude::ScalingInt;
    ///
    /// assert_eq!(100u8.scale_video(255), 100u8.scale(255)); // same as scale8...
    /// assert_ne!(1u8.scale_video(1),  1u8.scale(1));  // Except scale8() == 0
    /// ```
    fn scale_video(self, other: Self) -> Self;

    /// Dims an integer.
    ///
    /// The eye does not respond in a linear way to light. High speed PWM'd LEDs at 50% duty cycle
    /// appear far brighter then the 'half as bright' you might expect.
    ///
    /// If you want your midpoint brightness level (for `u8`, that'd be 128) to appear half as
    /// bright as 'full' brightness (255 for `u8`), you have to apply a dimming function.
    ///
    /// # Example
    ///
    /// ```
    /// use cichlid::prelude::ScalingInt;
    ///
    /// let full_brightness: u8 = 255;
    /// assert_eq!(255, full_brightness.dim_raw());
    ///
    /// let half_brightness: u8 = full_brightness / 2;
    /// assert_eq!(63, half_brightness.dim_raw());
    /// ```
    fn dim_raw(self) -> Self;

    /// Dims in video mode.
    ///
    /// This is the same as `dim_raw`, but the output of this function will only be zero if the
    /// input is zero.
    ///
    /// # Example
    ///
    /// ```
    /// use cichlid::prelude::ScalingInt;
    ///
    /// assert_eq!(255u8.dim_raw(), 255u8.dim_video());
    /// assert_ne!(30u8.dim_raw(), 30u8.dim_video());
    /// ```
    fn dim_video(self) -> Self;

    /// Dims an integer linearly.
    ///
    /// This is the same as `dim_raw`, but when `x < (Self::MAX / 2)`, the value is simply halved.
    /// The output will only be zero if the input is zero.
    fn dim_lin(self) -> Self;

    /// Inverse of the `dim_raw` function, brightens a value.
    fn brighten_raw(self) -> Self;

    /// Inverse of the `dim_video` function, brightens a value.
    fn brighten_video(self) -> Self;

    /// Linear version of the `brighten8_raw`, that halves for values < `Self::MAX / 2`.
    ///
    /// Notably, this is the relative inverse of `dim_lin`.
    fn brighten_lin(self) -> Self;

    /// Blends self with another integer by the fraction `amount_of_b`.
    fn blend(self, b: Self, amount_of_b: Self) -> Self;
}

macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

// nscaling macro
macro_rules! impl_nscale_ops {
    ($t:tt, $up:tt, $shift:expr, $mscaler:expr, $($element:tt),*) => {
         let scaler: $up = 1 as $up + $up::from($mscaler);
         $( *$element = (((*$element as $up) * scaler) >> $shift) as $t; )*
    };
}

macro_rules! impl_scale_ops { ($t:tt, $up:tt, $shift:expr, $max:expr) => (
    doc_comment!{concat!(
        "Scale a `", stringify!($t), "` by another."),
        #[inline(always)]
        pub const fn scale(i: $t, scale: $t) -> $t {
            (((i as $up) * (1 as $up + scale as $up)) >> $shift) as $t
        }
    }

    doc_comment!{concat!(
        "Scale a `", stringify!($t), "` by another, but in video mode.",
        "\n\n",
        "Video scaling guarantees the output of this function will only be zero",
        "if-and-only-if at least one of the inputs are zero."),
        #[inline]
        pub const fn scale_video(i: $t, scale: $t) -> $t {
            let x: $t = (((i as $up) * (scale as $up)) >> $shift) as $t;
            let correction_int: $t = (i != 0) as $t;
            let correction_scale: $t = (scale != 0) as $t;
            let correction: $t = correction_int & correction_scale;
            x + correction as $t
        }}

    doc_comment!{concat!("Dims a `", stringify!($t), "`."),
        #[inline(always)]
        pub const fn dim_raw(x: $t) -> $t {
            scale(x, x)
        }}

    doc_comment!{concat!(
        "Dims a `", stringify!($t), "` in video mode.",
        "\n\n",
        "Similar to `scale_video`, the output will only be zero if the input",
        "is also zero."),
        #[inline(always)]
        pub const fn dim_video(x: $t) -> $t {
            scale_video(x, x)
        }}

    doc_comment!{concat!(
        "Dims a `", stringify!($t), "` similar to `dim_raw`, but linearly below a threshold.",
        "\n\n",
        "When the input is less than equal to`", stringify!($max / 2), "`, the output is dimmed ",
        "by halving."),
        #[inline]
        pub const fn dim_lin(x: $t) -> $t {
            const UPPER_BITS: $t = (1 << ($shift - 1));
            let use_lin = (x & UPPER_BITS) != 0;
            let scale_x_reg = (use_lin as $t) * scale(x, x);
            let scale_x_lin = (!use_lin as $t) * (x.wrapping_add(1) / 2);
            // This is just a hack to be able to use const fns.
            scale_x_reg.wrapping_add(scale_x_lin)
        }}

    doc_comment!{concat!(
        "Brightens a `", stringify!($t), "`.",
        "\n\n",
        "This is the inverse of `dim_raw`."),
        #[inline]
        pub const fn brighten_raw(x: $t) -> $t {
            let ix = $max - x;
            $max - dim_raw(ix)
        }}

    doc_comment!{concat!(
        "Brightens a `", stringify!($t), "` but in video mode.",
        "\n\n",
        "This is the inverse of `dim_video`."),
        #[inline]
        pub const fn brighten_video(x: $t) -> $t {
            let ix = $max - x;
            $max - dim_video(ix)
        }}

    doc_comment!{concat!(
        "Brightens a `", stringify!($t), "`, but linearly below a threshold.",
        "\n\n",
        "This is the inverse of `dim_lin`."),
        #[inline]
        pub const fn brighten_lin(x: $t) -> $t {
            let ix = $max - x;
            $max - dim_lin(ix)
        }}

    doc_comment!{concat!(
        "Scales a single `", stringify!($t), "` in place."),
        #[inline(always)]
        pub fn nscale(int: &mut $t, scaler: $t) {
            *int = scale(*int, scaler);
        }}

    doc_comment!{concat!(
        "Inplace scaling for two `", stringify!($t), "`'s by the same value."),
        #[inline(always)]
        pub fn nscale_x2(int_1: &mut $t, int_2: &mut $t, scaler: $t) {
            impl_nscale_ops!($t, $up, $shift, scaler, int_1, int_2);
        }}

    doc_comment!{concat!(
        "Inplace scaling for three `", stringify!($t), "`'s by the same value."),
        #[inline]
        pub fn nscale_x3(int_1: &mut $t, int_2: &mut $t, int_3: &mut $t, scaler: $t) {
            impl_nscale_ops!($t, $up, $shift, scaler, int_1, int_2, int_3);
        }}

    doc_comment!{concat!(
        "Inplace scaling for four `", stringify!($t), "`'s by the same value."),
        #[inline]
        pub fn nscale_x4(int_1: &mut $t, int_2: &mut $t, int_3: &mut $t, int_4: &mut $t, scaler: $t) {
            impl_nscale_ops!($t, $up, $shift, scaler, int_1, int_2, int_3, int_4);
        }}


    doc_comment!{concat!(
        "Blends a `", stringify!($t), "`another integer by the fraction `amount_of_b`."),
        #[inline]
        pub const fn blend(a: $t, b: $t, amount_of_b: $t) -> $t {
            let amount_of_a: $up = ($max - amount_of_b) as $up;
            let mut partial: $up = 0;
            partial += a as $up * amount_of_a as $up;
            partial += a as $up;
            partial += b as $up * amount_of_b as $up;
            partial += b as $up;
            (partial >> $shift) as $t
        }}
    )
}

// Re exports a function name to be used through another.
//
// Great for creating shimmy traits with already made functions underneath.
macro_rules! impl_scaling_trait_rename {
    ($t:tt, $fname:ident) => (
        #[inline(always)]
        fn $fname(self) -> $t {
            $fname(self)
        }
    );
    ($t:tt, $param:ident, $fname:ident) => (
        #[inline(always)]
        fn $fname(self, $param: $t) -> $t {
            $fname(self, $param)
        }
    );

    ($t:tt, $param_1:ident, $param_2:ident, $fname:ident) => (
        #[inline(always)]
        fn $fname(self, $param_1: $t, $param_2: $t) -> $t {
            $fname(self, $param_1, $param_2)
        }
    );
}

macro_rules! impl_scaling_trait {
    ($t:tt) => {
        impl crate::math::ScalingInt for $t {
            impl_scaling_trait_rename!($t, other, scale);
            impl_scaling_trait_rename!($t, other, scale_video);
            impl_scaling_trait_rename!($t, dim_raw);
            impl_scaling_trait_rename!($t, dim_video);
            impl_scaling_trait_rename!($t, dim_lin);
            impl_scaling_trait_rename!($t, brighten_raw);
            impl_scaling_trait_rename!($t, brighten_video);
            impl_scaling_trait_rename!($t, brighten_lin);
            impl_scaling_trait_rename!($t, b, amount_of_b, blend);
        }
    };
}

mod math_u8_impls {
    //! Math functions for `u8`s. Includes scaling, dimming, brightening.
    //!
    //! Better documentation for these functions can be found under [`ScalingInt`].
    //!
    //! [`ScalingInt`]: ../trait.ScalingInt.html
    impl_scale_ops!(u8, u16, 8, 255);
    impl_scaling_trait!(u8);
}

mod math_u16_impls {
    //! Math functions for `u16`s. Includes scaling, dimming, brightening.
    //!
    //! Better documentation for these functions can be found under [`ScalingInt`].
    //!
    //! [`ScalingInt`]: ../trait.ScalingInt.html
    impl_scale_ops!(u16, u32, 16, 65535);
    impl_scaling_trait!(u16);
}
