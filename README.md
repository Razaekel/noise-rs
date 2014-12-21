noise-rs
========

[![Build Status](https://travis-ci.org/bjz/noise-rs.png)](https://travis-ci.org/bjz/noise-rs)


Procedural noise generation library for Rust.

API
===

This is the proposed new API for noise-rs. Thus far, only a fraction of it is implemented: Seed, Point2d, Point3d, Point4d, perlin2d, perlin3d and perlin4d.

    struct Seed { ... }
    Seed {
        fn new(seed: u32) -> Seed;
    }

    pub type Point2d<T> = [T, ..2];
    pub type Point3d<T> = [T, ..3];
    pub type Point4d<T> = [T, ..4];

    fn perlin2d<T: Float>(seed: &Seed, point: Point2d<T>) -> f32;
    fn perlin3d<T: Float>(seed: &Seed, point: Point3d<T>) -> f32;
    fn perlin3d<T: Float>(seed: &Seed, point: Point4d<T>) -> f32;

    fn open_simplex2d<T: Float>(seed: &Seed, point: Point2d<T>) -> f32;
    fn open_simplex3d<T: Float>(seed: &Seed, point: Point3d<T>) -> f32;
    fn open_simplex4d<T: Float>(seed: &Seed, point: Point4d<T>) -> f32;

    fn worley2d_points<T: Float>(seed: &Seed, point: Point2d<T>) -> [Point2d<T>, ..9];
    fn worley3d_points<T: Float>(seed: &Seed, point: Point3d<T>) -> [Point3d<T>, ..27];
    fn worley4d_points<T: Float>(seed: &Seed, point: Point4d<T>) -> [Point4d<T>, ..81];

    fn worley2d_nearest_point<T: Float>(seed: &Seed, point: Point2d<T>) -> f32;
    fn worley3d_nearest_point<T: Float>(seed: &Seed, point: Point3d<T>) -> f32;
    fn worley4d_nearest_point<T: Float>(seed: &Seed, point: Point4d<T>) -> f32;

    fn worley2d_nearest_edge<T: Float>(seed: &Seed, point: Point2d<T>) -> f32;
    fn worley3d_nearest_edge<T: Float>(seed: &Seed, point: Point3d<T>) -> f32;
    fn worley4d_nearest_edge<T: Float>(seed: &Seed, point: Point4d<T>) -> f32;

    fn worley2d_manhattan_point<T: Float>(seed: &Seed, point: Point2d<T>) -> f32;
    fn worley3d_manhattan_point<T: Float>(seed: &Seed, point: Point3d<T>) -> f32;
    fn worley4d_manhattan_point<T: Float>(seed: &Seed, point: Point4d<T>) -> f32;

    fn worley2d_manhattan_edge<T: Float>(seed: &Seed, point: Point2d<T>) -> f32;
    fn worley3d_manhattan_edge<T: Float>(seed: &Seed, point: Point3d<T>) -> f32;
    fn worley4d_manhattan_edge<T: Float>(seed: &Seed, point: Point4d<T>) -> f32;

    fn brownian2d<T: Float, F>(seed: &Seed, point: Point2d<T>, noise_func: F, wavelength: f32, octaves: u32) -> f32
        where F: Fn(Seed, point2d) -> f32;
    fn brownian3d<T: Float, F>(seed: &Seed, point: Point3d<T>, noise_func: F, wavelength: f32, octaves: u32) -> f32
        where F: Fn(Seed, point3d) -> f32;
    fn brownian4d<T: Float, F>(seed: &Seed, point: Point4d<T>, noise_func: F, wavelength: f32, octaves: u32) -> f32
        where F: Fn(Seed, point4d) -> f32;
