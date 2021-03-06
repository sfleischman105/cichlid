//! A `no-std` compatible library for fast color math, intended for use in programming
//! addressable LEDs.
//!
//! Currently this library is geared toward use in embedded systems, but does contain useful
//! APIs that are more generally useful.
//!
//! - **Fast `u8` and `u16` math** — Cichlid includes functions for scaling, dimming, and
//!    brightening single and double byte values. Basic trigonometric functions (sine, cosine)
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
//! # Examples
//!
//! General Color operations:
//!
//! ```
//! use cichlid::ColorRGB;
//!
//! let red = ColorRGB::Red;
//! let blue = ColorRGB::Blue;
//! let mut purple = red + blue;
//! assert_eq!(purple, ColorRGB::new(255, 0, 255));
//!
//! purple.scale(128); // Scale by half
//! assert_eq!(purple, ColorRGB::new(128, 0, 128));
//!
//! purple *= 2;  // Multiple all components by two
//! assert_eq!(purple, red + blue);
//! ```
//!
//! Using `HSV` (Hue, Saturation, Value) and converting to `ColorRGB`:
//!
//! ```
//! use cichlid::{HSV, ColorRGB, prelude::*};
//!
//! let red_hsv = HSV::new(0, 255, 255);
//! let red_rgb = ColorRGB::from(red_hsv);
//! assert_eq!(red_rgb, ColorRGB::Red);
//! ```
//!
//! Creating a gradient is very easy, simply import the trait and call the method:
//!
//! ```
//! use cichlid::{HSV, ColorRGB, GradientDirection, prelude::*};
//! let mut colors = [ColorRGB::Black; 100];
//!
//! let start = HSV::new(0, 255, 255);
//! let end = HSV::new(100, 255, 180);
//! colors.gradient_fill(start, end, GradientDirection::Longest);
//! ```
//!
//! We can also create rainbows from both a step size, as well as a forming a complete rainbow.
//!
//! ```
//! use cichlid::{HSV, ColorRGB, GradientDirection, prelude::*};
//! let mut colors = [ColorRGB::Black; 256];
//!
//! let start_hue: u8 = 0;
//! let hue_delta: u16 = (1 << 8);
//!
//! colors.rainbow_fill(start_hue, hue_delta); // From step size
//! colors.rainbow_fill_single_cycle(start_hue); // Complete rainbow
//! ```
//!
//! # no-std
//!
//! To use in a `no-std` environment, simply add the following to your project's `cargo.toml`:
//!
//! ```ignore
//! [dependencies.cichlid]
//! version = "*"
//! features = ["no-std"]
//! ```
//!
//! # Low memory usage
//!
//! The `low-mem` feature creates a binary that is smaller due to relying less on in memory
//! tables, preferring direct computation instead. The only drawback of this is a slight
//! speed decrease.
//!
//! # Nightly features
//!
//! To use some unstable nightly features and optimizations, use the `nightly` feature flag.
//!
//! # Acknowledgements
//!
//! This library takes heavy inspiration and code-reuse from
//! [FastLED](https://github.com/FastLED/FastLED), an Arduino library for talking to addressable
//! LEDs.

// TODO: SERDE
#![cfg_attr(feature = "no-std", no_std)]
#![cfg_attr(feature = "nightly", feature(link_llvm_intrinsics))]
//#![feature(link_llvm_intrinsics)]

macro_rules! mk_rgb {
    ($r:expr, $g:expr, $b:expr) => {
        crate::rgb::ColorRGB::new($r, $g, $b)
    };
    ($f_rgb:expr) => {
        crate::rgb::ColorRGB::from($f_rgb)
    };
}

//macro_rules! HSV {
//    ($h:expr, $s:expr, $v:expr) => {crate::hsv::HSV::new($h, $s, $v)};
//    ($h:expr) => {crate::hsv::HSV::new($h, 255, 255)};
//}

pub mod color_codes;
mod color_util;
mod hsv;
pub mod math;
mod power_mgmt;
mod rgb;

pub use crate::color_util::GradientDirection;
pub use crate::hsv::HSV;

pub use crate::power_mgmt::{DefaultPowerEstimator, PowerEstimator};
pub use crate::rgb::ColorRGB;
pub use crate::color_util::gradient::{hsv_gradient,rgb_gradient};


pub mod prelude {
    //! Easy importing of integer and color auto traits.

    pub use crate::math::ScalingInt;
    pub use crate::math::Trig;

    pub use crate::color_util::ColorIterMut;
    pub use crate::color_util::ColorSliceMut;

    pub use crate::color_util::GradientFill;
    pub use crate::color_util::GradientFillToInclusive;

    pub use crate::color_util::GradientFillRGB;
    pub use crate::color_util::GradientFillRGBToInclusive;

    pub use crate::color_util::RainbowFill;
    pub use crate::color_util::RainbowFillSingleCycle;
}
