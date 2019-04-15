//! Holds various math functions.
pub(crate) mod lerp;
pub mod scale;
pub mod trig;

/// Trigonometric functions on unsigned integers.
pub trait Trig<Signed> {
    fn sin(self) -> Signed;
    fn cos(self) -> Signed;
}

/// Scaling, Dimming, and Brightening functions for integers.
pub trait Scaling {
    /// Scales self by a second one (`scale`), which is treated as the numerator
    /// of a fraction whose denominator is `Self::MAX`.
    ///
    /// In other words, it computes `i * (scale / Self::MAX)`
    ///
    /// # Example
    ///
    /// ```
    /// use cichlid::math::scale::scale8;
    ///
    /// assert_eq!(scale8(100, 255), 100); // 100 * 1.0
    /// assert_eq!(scale8(100, 0), 0); // 100 * 0.0
    /// assert_eq!(scale8(100, 255 / 2), 50); // 100 * 0.5
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
    /// use cichlid::math::scale::{scale8_video, scale8};
    ///
    /// assert_eq!(scale8_video(100, 255), scale8(100, 255)); // same as scale8...
    /// assert_ne!(scale8_video(1, 1),  scale8(1, 1));  // Except scale8() == 0
    /// ```
    fn scale_video(self, other: Self) -> Self;

    /// Dims self.
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
    /// use cichlid::math::scale::dim8_raw;
    ///
    /// let full_brightness: u8 = 255;
    /// assert_eq!(255, dim8_raw(full_brightness));
    ///
    /// let half_brightness: u8 = full_brightness / 2;
    /// assert_eq!(63, dim8_raw(half_brightness));
    /// ```
    fn dim_raw(self) -> Self;

    /// Dims in video mode.
    ///
    /// This is the same as `dim_raw`, but the output of this function will only be zero if the
    /// parameter is zero.
    ///
    /// # Example
    ///
    /// ```
    /// use cichlid::math::scale::{dim8_raw,dim8_video};
    ///
    /// assert_eq!(dim8_raw(255), dim8_video(255));
    /// assert_ne!(dim8_raw(30), dim8_video(30));
    /// ```
    fn dim_video(self) -> Self;


    /// Dims linearly.
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
    /// It is also the inverse of `dim_lin`.
    fn brighten_lin(self) -> Self;

    /// Blends self with another integer by the fraction `amount_of_b`.
    fn blend(self, b: Self, amount_of_b: Self) -> Self;
}
