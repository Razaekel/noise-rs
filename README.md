# noise-rs

[![Build Status](https://travis-ci.org/brendanzab/noise-rs.svg?branch=master)](https://travis-ci.org/brendanzab/noise-rs)

A procedural noise generation library for Rust.

```rust
use noise::{Brownian3, Seed};

let seed = Seed::new(12);
let noise = Brownian3::new(noise::perlin3, 4).wavelength(32.0);
let val = noise.apply(&seed, &[42.0, 37.0, 2.0]);
```

## API

### Gradient Noise

Gradient noise produces a smooth, continuous value over space. It's achieved by
dividing space into regions, and placing a random gradient at each vertex, then
blending between those gradients.

#### Perlin noise

A very fast and reasonable quality gradient noise:

- `perlin2`
- `perlin3`
- `perlin4`

#### OpenSimplex noise

A slower but higher quality form of gradient noise:

- `open_simplex2`
- `open_simplex3`
- `open_simplex4`

### Value Noise

Value noise (sometimes mistaken with gradient noise) produces lower quality
smooth noise. It exhibits pronounced grid artifacts, but can be slightly faster
than gradient noise. Benchmarks show it's about 1.2–1.3× faster than Perlin noise.

Cell neighbours are blended using a weighted S-curve linear interpolation
method. This removes any discontinuities across grid edges.

- `value2`
- `value3`
- `value4`

### Fractional Brownian Motion

A way of combining multiple octaves of a noise function to create a richer and
more varied output:

- `Brownian2`
- `Brownian3`
- `Brownian4`

### Cell Noise

Cell noise, also called worley noise or voronoi noise, is based on dividing
space into cells based on proximity to a random set of seed points. In this
API, this is accomplished in three categories.

#### Noise Functions

These are the actual noise functions, which just take a coordinate and return
a value. They use the functions from the later categories to generate the most
common and useful types of cell noise. Most of the time, these are the only
ones you'll be interested in.

These functions, when given a point, will return a value generated from the
cell coordinates of the nearest seed point, so all points within the same
cell will return the same value:

- `cell2_value`
- `cell3_value`
- `cell4_value`

These functions, when given a point, will return the range to the nearest seed
point:

- `cell2_range`
- `cell3_range`
- `cell4_range`

These functions, when given a point, will return the range to the nearest cell
border. This is accomplished by subtracting the range to the nearest point from
the range to the second nearest point:

- `cell2_range_inv`
- `cell3_range_inv`
- `cell4_range_inv`

These functions, when given a point, will return a value generated from the
cell coordinates of the nearest seed point, but using manhattan distance.
This results in more squared-off cells:

- `cell2_value_manhattan`
- `cell3_value_manhattan`
- `cell4_value_manhattan`

These functions, when given a point, will return range to the nearest seed
point, but using manhattan distance:

- `cell2_manhattan`
- `cell3_manhattan`
- `cell4_manhattan`

These functions, when given a point, will return the range to the nearest cell
border, but in manhattan distance:

- `cell2_manhattan_inv`
- `cell3_manhattan_inv`
- `cell4_manhattan_inv`

#### Range Functions

These are the set of functions for determining range between two points, which
are used to determine which seed point is actually closest to the test point.

Calculate the square of the euclidian range between two points. This is what
one normally thinks of as range. It's squared because that's cheaper to
calculate, and we only actually care about which distance is greater in
this context:

- `range_sqr_euclidian2`
- `range_sqr_euclidian3`
- `range_sqr_euclidian4`

Calculate the manhattan range between two points. This is the sum of the
coordinates in the offset vector, equivalent to the distance one would walk on
the Manhattan streets between two points:

- `range_manhattan2`
- `range_manhattan3`
- `range_manhattan4`

#### Seed Point Functions

These are the set of functions for returning the nearest point, nearest 2
points, or the cell of the nearest point. If you want to do something a little
unusual with the cell noise, you can call these directly to operate on this
intermediate data.

These functions, when given a point, will return the nearest seed point, and
the range to that point:

- `cell2_seed_point`
- `cell3_seed_point`
- `cell4_seed_point`

These functions, when given a point, will return the nearest 2 seed points, and
the range to those points. The first point is the nearest one, and the second
point the second nearest:

- `cell2_seed_2_points`
- `cell3_seed_2_points`
- `cell4_seed_2_points`

These functions, when given a point, will return the cell of the nearest seed point:

- `cell2_seed_cell`
- `cell3_seed_cell`
- `cell4_seed_cell`
