#[cfg(feature = "images")]
use noise::utils::{
    NoiseImage,
    NoiseMap,
};

#[allow(dead_code)]
#[cfg(feature = "images")]
pub fn write_example_to_file(map: &NoiseMap, filename: &str) {
    use std::{
        fs,
        path::Path,
    };

    let target_dir = Path::new("example_images/");

    if !target_dir.exists() {
        fs::create_dir(target_dir).expect("failed to create example_images directory");
    }

    let target = target_dir.join(Path::new(filename));

    map.write_to_file(target.to_str().unwrap())
}

#[allow(dead_code)]
#[cfg(feature = "images")]
pub fn write_image_to_file(image: &NoiseImage, filename: &str) {
    use std::{
        fs,
        path::Path,
    };

    let target_dir = Path::new("example_images/");

    if !target_dir.exists() {
        fs::create_dir(target_dir).expect("failed to create example_images directory");
    }

    let target = target_dir.join(Path::new(filename));

    image.write_to_file(target.to_str().unwrap())
}

#[allow(dead_code)]
fn main() {
    println!("This is not an example")
}
