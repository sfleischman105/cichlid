//! A `no-std` compatible library for fast color math, intended for use in programming
//! addressable LEDs.
//!
//! Currently this library is geared toward use in embedded systems, but does contain useful
//! APIs that are more generally useful.
//!
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
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/cichlid) and can be
//! used by adding `cichlid` to the dependencies in your project's `Cargo.toml`.
//!
//!
//! # `no-std` Usage
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

#![cfg_attr(feature="no-std", no_std)]

pub mod color_codes;

pub mod scale;
pub mod trig;
pub mod rgb;
pub mod hsv;
pub mod gradient;
pub mod power_mgmt;

pub use crate::rgb::ColorRGB;
pub use crate::hsv::HSV;
//pub use crate::gradient::{hsv_gradient, hsv_gradient_inc_end, GradientDirection};
pub use crate::power_mgmt::{DefaultPowerEstimator,PowerEstimator};