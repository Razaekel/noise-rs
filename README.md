# noise-rs

[![Build Status](https://travis-ci.org/Razaekel/noise-rs.svg?branch=master)](https://travis-ci.org/Razaekel/noise-rs)
[![Documentation](https://img.shields.io/badge/documentation-online-blue.svg)](https://docs.rs/noise/)
[![Crates.io](https://img.shields.io/crates/v/noise.svg)](https://crates.io/crates/noise)

A procedural noise generation library for Rust.

[Documentation](https://docs.rs/noise/)

```rust
use noise::{Fbm, NoiseFn};

let fbm = Fbm::new();

let val = fbm.get([42.0, 37.0, 2.0]);
```

## API

### Gradient Noise

Gradient noise produces a smooth, continuous value over space. It's achieved by
dividing space into regions, and placing a random gradient at each vertex, then
blending between those gradients.

#### Perlin noise

A very fast and reasonable quality gradient noise:

- `Perlin::new()`

#### OpenSimplex noise

A slower but higher quality form of gradient noise:

- `OpenSimplex::new()`

### Value Noise

Value noise (sometimes mistaken with gradient noise) produces lower quality
smooth noise. It exhibits pronounced grid artifacts, but can be slightly faster
than gradient noise. Benchmarks show it's about 1.2–1.3× faster than Perlin noise.

Cell neighbours are blended using a weighted S-curve linear interpolation
method. This removes any discontinuities across grid edges.

- `Value::new()`

### Fractional Brownian Motion

A way of combining multiple octaves of a noise function to create a richer and
more varied output:

- `Fbm::new()`

### Worley Noise

Named after Steven Worley, and also called Voronoi noise, is based on dividing
space into cells based on proximity to a random set of seed points.

- `Worley::new()`

#### Noise Functions

These are the actual noise functions, which just take a coordinate using `get()` and return
a value. They can be chained together when declared, creating very complex noise results.

See the individual function pages for their descriptions, and the examples for their usage.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
