//! A `no-std` compatible library for fast color math, intended for use in programming
//! addressable LEDs.
//!
//! Currently this library is geared toward use in embedded systems, but does contain useful
//! APIs that are more generally useful.
//!
//! - **Fast `u8` and `u16` math** — Chiclid includes functions for scaling, dimmming, and
//!    brightening single and double byte values. Basic trigometric functions (sine, cosine)
//!    are implemented as well.
//!
//! - **HSV and RGB support** — Full control over each color is provided by the `HSV` and
//!   `ColorRGB` structures. Different means of converting from `HSV` to `ColorRGB` are also
//!   implemented.
//!
//! - **Axial (Two Point) Color Gradients** — Create smooth transitions between any two colors
//!   for any number of steps.
//!
//! - **Power Consumption Estimating** — Estimating power requirements can be done with
//!   structs implementing the `PowerEstimator` trait.
//!
//! This Library is still in its infancy, and as such there may be a lack of documentation and
//! vigorous testing.
//!
//!
//! # `no-std`
//!
//! To use in a `no-std` enviorment, simply add the following to your project's `cargo.toml`:
//!
//! ```ignore
//! [dependencies.cichlid]
//! version = "*"
//! features = ["no-std"]
//! ```
//!
//! # Acknowledgements
//!
//! This library takes heavy inspiration and code-reuse from
//! [FastLED](https://github.com/FastLED/FastLED), an Arduino library for talking to addressable
//! LEDs.

// TODO: SERDE
#![cfg_attr(feature = "no-std", no_std)]
pub mod color_codes;

pub mod color_util;
pub mod hsv;
pub mod math;
pub mod power_mgmt;
pub mod rgb;

pub use crate::color_util::GradientDirection;
pub use crate::hsv::HSV;
pub use crate::power_mgmt::{DefaultPowerEstimator, PowerEstimator};
pub use crate::prelude::*;
pub use crate::rgb::ColorRGB;

pub mod prelude {
    //! Easy importing of color auto traits.
    pub use crate::color_util::Blur as _;
    pub use crate::color_util::GradientFill as _;
    pub use crate::color_util::GradientFillRGB as _;
    pub use crate::color_util::GradientFillRGBToInclusive as _;
    pub use crate::color_util::GradientFillToInclusive as _;
    pub use crate::color_util::RainbowFill as _;
    pub use crate::color_util::RainbowFillSingleCycle as _;
}
