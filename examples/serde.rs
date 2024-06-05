use std::{fs::File, io::Write};

use noise::{utils::*, Perlin};
use serde::{self, Serialize};
mod utils;

fn main() {
    let perlin = Perlin::default();

    let map = &PlaneMapBuilder::new(perlin).build();

    let serialized = serde_json::ser::to_string(map).expect("error serializing");

    let mut file = File::create("examples/file.json").expect("error opening file");

    let _ = file.write(serialized.as_bytes());
}
