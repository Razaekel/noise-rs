noise-rs
========

[![Build Status](https://travis-ci.org/bjz/noise-rs.png)](https://travis-ci.org/bjz/noise-rs)


Procedural noise generation library for Rust.

API
===

All the noise functions require a seed. In most circumstances, you'll only want to create one of these and use it everywhere.
~~~rust
struct Seed { ... }
impl Seed {
    fn new(seed: u32) -> Seed;
}
~~~

Point types are just fixed-sized arrays, which any linear algebra library should be able to convert to.
~~~rust
pub type Point2<T> = [T; 2];
pub type Point3<T> = [T; 3];
pub type Point4<T> = [T; 4];
~~~

Gradient Noise
--------------
Gradient noise produces a smooth, continuous value over space. It's achieved by dividing space into regions, and placing a random gradient at each vertex, then blending between those gradients.

**Perlin Noise**

Perlin noise is a very fast and reasonable quality gradient noise.
~~~rust
fn perlin2<T: Float>(seed: &Seed, point: &Point2<T>) -> T;
fn perlin3<T: Float>(seed: &Seed, point: &Point3<T>) -> T;
fn perlin4<T: Float>(seed: &Seed, point: &Point4<T>) -> T;
~~~

**OpenSimplex Noise**

OpenSimplex noise is a slower but higher quality form of gradient noise.
~~~rust
fn open_simplex2<T: Float>(seed: &Seed, point: &Point2<T>) -> T;
fn open_simplex3<T: Float>(seed: &Seed, point: &Point3<T>) -> T;
~~~

**Fractional Brownian Motion**

Fractional Brownian Motion is a way of combining multiple octaves of a noise function to create a richer and more varied output. It can theoretically be used with any noise function, but it tends to only produce good results with gradient noise functions.

Example:
~~~rust
let brownian = Brownian2::new(perlin2::<f32>, 4).wavelength(32.0);
let val = brownian(&seed, [42.0, 37.0]);
~~~

Brownian has a lot of properties, which are as follows:
* function - The noise function to call.
* octaves - The number of times to call it.
* wavelength - The amount to scale the coordinates on the first call. Defaults to 1.0.
* frequency - The inverse of the wavelength, provided for convenience. Defaults to 1.0.
* persistence - The rate at which the amplitude is reduced from one call to the next. Defaults to 0.5.
* lacunarity - The rate at which the frequency increases from one call to the next. Defaults to 2.0.

All of the calls to the noise function are added to gether into the return value.

~~~rust
struct Brownian2<T: Float, F: Fn(&Seed, &Point2<T>) -> T> { ... }
impl<T, F> Brownian2<T, F> {
    pub fn new(function: F, octaves: usize) -> Brownian2<T, F>;
    pub fn function(self, function: F) -> Brownian2<T, F>;
    pub fn octaves(self, octaves: usize) -> Brownian2<T, F>;
    pub fn wavelength(self, wavelength: T) -> Brownian2<T, F>;
    pub fn frequency(self, frequency: T) -> Brownian2<T, F>;
    pub fn persistence(self, persistence: T) -> Brownian2<T, F>;
    pub fn lacunarity(self, lacunarity: T) -> Brownian2<T, F>;
}
impl<T, F> Fn(&Seed, &Point2<T>) -> T for Brownian2<T, F> { ... }

struct Brownian3<T: Float, F: Fn(&Seed, &Point3<T>) -> T> { ... }
impl<T, F> Brownian3<T, F> {
    pub fn new(function: F, octaves: usize) -> Brownian3<T, F>;
    pub fn function(self, function: F) -> Brownian3<T, F>;
    pub fn octaves(self, octaves: usize) -> Brownian3<T, F>;
    pub fn wavelength(self, wavelength: T) -> Brownian3<T, F>;
    pub fn frequency(self, frequency: T) -> Brownian3<T, F>;
    pub fn persistence(self, persistence: T) -> Brownian3<T, F>;
    pub fn lacunarity(self, lacunarity: T) -> Brownian3<T, F>;
}
impl<T, F> Fn(&Seed, &Point3<T>) -> T for Brownian3<T, F> { ... }

struct Brownian4<T: Float, F: Fn(&Seed, &Point4<T>) -> T> { ... }
impl<T, F> Brownian3<T, F> {
    pub fn new(function: F, octaves: usize) -> Brownian3<T, F>;
    pub fn function(self, function: F) -> Brownian3<T, F>;
    pub fn octaves(self, octaves: usize) -> Brownian3<T, F>;
    pub fn wavelength(self, wavelength: T) -> Brownian3<T, F>;
    pub fn frequency(self, frequency: T) -> Brownian3<T, F>;
    pub fn persistence(self, persistence: T) -> Brownian3<T, F>;
    pub fn lacunarity(self, lacunarity: T) -> Brownian3<T, F>;
}
impl<T, F> Fn(&Seed, &Point4<T>) -> T for Brownian4<T, F> { ... }
~~~

Cell Noise
----------

Cell noise, also called worley noise or voronoi noise, is based on dividing space into cells based on proximity to a random set of seed points. In this API, this is accomplished in three categories.

**Noise Functions**

These are the actual noise functions, which just take a coordinate and return a value. They use the functions from the later categories to generate the most common and useful types of cell noise. Most of the time, these are the only ones you'll be interested in.

These functions, when given a point, will return a value generated from the cell coordinates of the nearest seed point, so all points within the same cell will return the same value.
~~~rust
fn cell2_value<T: Float>(seed: &Seed, point: &Point2<T>) -> T;
fn cell3_value<T: Float>(seed: &Seed, point: &Point3<T>) -> T;
fn cell4_value<T: Float>(seed: &Seed, point: &Point4<T>) -> T;
~~~

These functions, when given a point, will return the range to the nearest seed point.
~~~rust
fn cell2_range<T: Float>(seed: &Seed, point: &Point2<T>) -> T;
fn cell3_range<T: Float>(seed: &Seed, point: &Point3<T>) -> T;
fn cell4_range<T: Float>(seed: &Seed, point: &Point4<T>) -> T;
~~~

These functions, when given a point, will return the range to the nearest cell border. This is accomplished by subtracting the range to the nearest point from the range to the second nearest point.
~~~rust
fn cell2_range_inv<T: Float>(seed: &Seed, point: &Point2<T>) -> T;
fn cell3_range_inv<T: Float>(seed: &Seed, point: &Point3<T>) -> T;
fn cell4_range_inv<T: Float>(seed: &Seed, point: &Point4<T>) -> T;
~~~

These functions, when given a point, will return a value generated from the cell coordinates of the nearest seed point, but using manhattan distance. This results in more squared-off cells.
~~~rust
fn cell2_value_manhattan<T: Float>(seed: &Seed, point: &Point2<T>) -> T;
fn cell3_value_manhattan<T: Float>(seed: &Seed, point: &Point3<T>) -> T;
fn cell4_value_manhattan<T: Float>(seed: &Seed, point: &Point4<T>) -> T;
~~~

These functions, when given a point, will return range to the nearest seed point, but using manhattan distance.
~~~rust
fn cell2_manhattan<T: Float>(seed: &Seed, point: &Point2<T>) -> T;
fn cell3_manhattan<T: Float>(seed: &Seed, point: &Point3<T>) -> T;
fn cell4_manhattan<T: Float>(seed: &Seed, point: &Point4<T>) -> T;
~~~

These functions, when given a point, will return the range to the nearest cell border, but in manhattan distance.
~~~rust
fn cell2_manhattan_inv<T: Float>(seed: &Seed, point: &Point2<T>) -> T;
fn cell3_manhattan_inv<T: Float>(seed: &Seed, point: &Point3<T>) -> T;
fn cell4_manhattan_inv<T: Float>(seed: &Seed, point: &Point4<T>) -> T;
~~~

**Range Functions**

These are the set of functions for determining range between two points, which are used to determine which seed point is actually closest to the test point.

Calculate the square of the euclidian range between two points. This is what one normally thinks of as range. It's squared because that's cheaper to calculate, and we only actually care about which distance is greater in this context.
~~~rust
fn range_sqr_euclidian2<T: Float>(p1: Point2<T>, p2: Point2<T>) -> T;
fn range_sqr_euclidian3<T: Float>(p1: Point3<T>, p2: Point3<T>) -> T;
fn range_sqr_euclidian4<T: Float>(p1: Point4<T>, p2: Point4<T>) -> T;
~~~

Calculate the manhattan range between two points. This is the sum of the coordinates in the offset vector, equivalent to the distance one would walk on the Manhattan streets between two points.
~~~rust
fn range_manhattan2<T: Float>(p1: Point2<T>, p2: Point2<T>) -> T;
fn range_manhattan3<T: Float>(p1: Point3<T>, p2: Point3<T>) -> T;
fn range_manhattan4<T: Float>(p1: Point4<T>, p2: Point4<T>) -> T;
~~~

**Seed Point Functions**

These are the set of functions for returning the nearest point, nearest 2 points, or the cell of the nearest point. If you want to do something a little unusual with the cell noise, you can call these directly to operate on this intermediate data.

These functions, when given a point, will return the nearest seed point, and the range to that point.
~~~rust
fn cell2_seed_point<T, F>(seed: &Seed, point: &Point2<T>, range: F) -> (Point2<T>, T)
    where T: Float, F: fn(Point2<T>, Point2<T>) -> T;
fn cell3_seed_point<T, F>(seed: &Seed, point: &Point3<T>, range: F) -> (Point3<T>, T)
    where T: Float, F: fn(Point3<T>, Point3<T>) -> T;
fn cell4_seed_point<T, F>(seed: &Seed, point: &Point4<T>, range: F) -> (Point4<T>, T)
    where T: Float, F: fn(Point4<T>, Point4<T>) -> T;
~~~

These functions, when given a point, will return the nearest 2 seed points, and the range to those points. The first point is the nearest one, and the second point the second nearest.
~~~rust
fn cell2_seed_2_points<T, F>(seed: &Seed, point: &Point2<T>, range: F) -> (Point2<T>, T, Point2<T>, T)
    where T: Float, F: fn(Point2<T>, Point2<T>) -> T;
fn cell3_seed_2_points<T, F>(seed: &Seed, point: &Point3<T>, range: F) -> (Point3<T>, T, Point3<T>, T)
    where T: Float, F: fn(Point3<T>, Point3<T>) -> T;
fn cell4_seed_2_points<T, F>(seed: &Seed, point: &Point4<T>, range: F) -> (Point4<T>, T, Point4<T>, T)
    where T: Float, F: fn(Point4<T>, Point4<T>) -> T;
~~~

These functions, when given a point, will return the cell of the nearest seed point.
~~~rust
fn cell2_seed_cell<T, F>(seed: &Seed, point: &Point2<T>, range: F) -> Point2<i64>
    where T: Float, F: fn(Point2<T>, Point2<T>) -> T;
fn cell3_seed_cell<T, F>(seed: &Seed, point: &Point3<T>, range: F) -> Point3<i64>
    where T: Float, F: fn(Point3<T>, Point3<T>) -> T;
fn cell4_seed_cell<T, F>(seed: &Seed, point: &Point4<T>, range: F) -> Point4<i64>
    where T: Float, F: fn(Point4<T>, Point4<T>) -> T;
~~~

Coming soon
-----------
Everything below this line is planned, but not yet implemented.

~~~rust
fn open_simplex4<T: Float>(seed: &Seed, point: &Point4<T>) -> T;
~~~
