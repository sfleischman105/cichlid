# Cichlid

[![cichlid crate](https://img.shields.io/crates/v/cichlid.svg)](https://crates.io/crates/cichlid)
[![cichlid docs](https://docs.rs/cichlid/badge.svg)](https://docs.rs/crate/cichlid/)

A simple Rust library for managing RGB colorings. Works with `no-std` environments as well.

Currently this library is geared toward use in embedded systems, but does contain useful
APIs that are more generally useful.

- **Fast `u8` and `u16` math** — Cichlid includes functions for scaling, dimming, and
   brightening single and double byte values. Basic trigonometric functions (sine, cosine)
   are implemented as well.

- **HSV and RGB support** — Full control over each color is provided by the `HSV` and
  `ColorRGB` structures. Different means of converting from `HSV` to `ColorRGB` are also
  implemented.

- **Axial (Two Point) Color Gradients** — Create smooth transitions between any two colors
  for any number of steps.

- **Power Consumption Estimating** — Estimating power requirements can be done with
  structs implementing the `PowerEstimator` trait.

This Library is still in its infancy, and as such there may be a lack of documentation and
vigorous testing.

## Examples

General Color operations:

```rust
use cichlid::*;

let red = ColorRGB::Red;
let blue = ColorRGB::Blue;
let mut purple = red + blue;
assert_eq!(purple, ColorRGB::new(255, 0, 255));

purple.scale(128); // Scale by half
assert_eq!(purple, ColorRGB::new(128, 0, 128));

purple *= 2;  // Multiple all components by two
assert_eq!(purple, red + blue);
```

Using `HSV` (Hue, Saturation, Value) and converting to `ColorRGB`:

```rust
use cichlid::*;

let red_hsv = HSV::new(0, 255, 255);
let red_rgb = ColorRGB::from(red_hsv);
assert_eq!(red_rgb, ColorRGB::Red);
```

Creating a gradient is very easy, simply import the trait and call the method:

```rust
use cichlid::*;
let mut colors = [ColorRGB::Black; 100];

let start = HSV::new(0, 255, 255);
let end = HSV::new(100, 255, 180);
colors.gradient_fill(start, end, GradientDirection::Longest);
```

Contributing
-------

Any and all contributions are welcome! Open up a PR to contribute some improvements. 
Look at the Issues tab to see what needs some help. 

  
License
-------
Cichlid is distributed under the terms of the MIT license. See LICENSE-MIT for details. 
Opening a pull requests is assumed to signal agreement with these licensing terms.