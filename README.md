<!-- PROJECT BADGES -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Github Link][github badge]][github link]
[![CI Status][ci badge]][ci link]
[![Documentation][docs badge]][docs link]
[![Version][crates.io version]][crates.io link]
[![MSRV][rust 1.51.0+ badge]][rust 1.51.0+ link]

<!-- PROJECT LOGO -->
<br />
<p align="center">
  <!-- a href="https://github.com/razaekel/noise-rs">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a-->

<h1 align="center">Noise-rs</h1>

  <p align="center">
    Procedural Noise Generation library <i>for Rust</i>
    <br />
    <a href="https://docs.rs/noise"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <!--a href="https://github.com/razaekel/noise-rs">View Demo</a>
    ·-->
    <a href="https://github.com/razaekel/noise-rs/issues">Report Bug</a>
    ·
    <a href="https://github.com/razaekel/noise-rs/issues">Request Feature</a>
  </p>
</p>

```toml
[dependencies]
noise = "0.8"
```

<!-- ABOUT THE PROJECT -->
## About The Project
Noise-rs is a Rust library to generate smoothly varying noise for textural use and graphical display.

Noise generators are contained in `NoiseFn` modules, and can be combined to make very complex noise results.

### Planetary Surface Example
![planet surface image]
![planet surface 4x]
![planet surface 16x]

### Gradient Noise

Gradient noise produces a smooth, continuous value over space. It's achieved by
dividing space into regions, placing a random gradient at each vertex, and then
blending between those gradients.

#### Noise Functions

These are the actual noise functions, which just take a coordinate using `get()` and return
a value. They can be chained together when declared, creating very complex noise results.

See the individual function pages for their descriptions, and the [examples][examples link] for their usage.

<!-- USAGE EXAMPLES -->
## Usage

```rust
use noise::Fbm;
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};

fn main() {
  let fbm = Fbm::new();

  PlaneMapBuilder::new(&fbm)
          .set_size(1000, 1000)
          .set_x_bounds(-5.0, 5.0)
          .set_y_bounds(-5.0, 5.0)
          .build()
          .write_to_file("fbm.png");
}
```
![FBM Noise][fbm image]

_For more examples, refer to the [Examples][examples link]_

<!-- ROADMAP -->
## Roadmap

See the [open issues][issues link] for a list of proposed features (and known issues).

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as below, without any
additional terms or conditions.

<!-- LICENSE -->
## License

Licensed under either of
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

<!-- CONTACT -->
## Contact

Project Link: [https://github.com/razaekel/noise-rs][github link]

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[ci link]: https://github.com/razaekel/noise-rs/actions
[ci badge]: https://img.shields.io/github/workflow/status/razaekel/noise-rs/CI?style=for-the-badge&logo=github-actions&logoColor=white
[crates.io link]: https://crates.io/crates/noise
[crates.io version]: https://img.shields.io/crates/v/noise.svg?style=for-the-badge&logo=rust
[docs link]: https://docs.rs/noise
[docs badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
[examples link]: /examples/
[fbm image]: /images/fbm.png
[github badge]: https://img.shields.io/badge/github-razaekel/noise--rs-8da0cb?style=for-the-badge&logo=github
[github link]: https://github.com/razaekel/noise-rs
[issues link]: https://github.com/razaekel/noise-rs/issues
[planet surface image]: /images/unscaledFinalPlanet.png
[planet surface 4x]: /images/unscaledFinalPlanet_4x_zoom.png
[planet surface 16x]: /images/unscaledFinalPlanet_16x_zoom.png
[rust 1.51.0+ badge]: https://img.shields.io/badge/rust-1.51.0+-93450a.svg?style=for-the-badge&logo=rust
[rust 1.51.0+ link]: https://blog.rust-lang.org/2021/03/25/Rust-1.51.0.html
