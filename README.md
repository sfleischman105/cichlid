### Cichlid

A simple Rust library for managing RGB colorings. Works with `no-std` environments as well.

Currently this library is geared toward use in embedded systems, but does contain useful
APIs that are more generally useful.

- **Fast `u8` and `u16` math** — Chiclid includes functions for scaling, dimmming, and
   brightening single and double byte values. Basic trigometric functions (sine, cosine)
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

Contributing
-------

Any and all contributions are welcome! Open up a PR to contribute some improvements. 
Look at the Issues tab to see what needs some help. 

  
License
-------
Cichlid is distributed under the terms of the MIT license. See LICENSE-MIT for details. 
Opening a pull requests is assumed to signal agreement with these licensing terms.