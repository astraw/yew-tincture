# Crate `yew-tincture` for the [Rust language](https://www.rust-lang.org/)

[![Crates.io](https://img.shields.io/crates/v/yew-tincture.svg)](https://crates.io/crates/yew-tincture)
[![Documentation](https://docs.rs/yew-tincture/badge.svg)](https://docs.rs/yew-tincture/)

📐 components for Yew web framework

## About

The crate implements reusable components for the [Yew](https://yew.rs) web
framework:

- `Button`: a button that emits a callback when clicked.
- `CheckboxLabel`: a checkbox with a label, emitting its new state on toggle.
- `TypedInput`: a text input that parses its contents as any `T: FromStr`,
  backed by a shareable `TypedInputStorage<T>`.

| yew-tincture | yew  |
|--------------|------|
| 0.4          | 0.23 |
| 0.3          | 0.22 |
| 0.2          | 0.21 |

The minimum supported Rust version is 1.100.

## Example app

`yew-tincture-test/` is a small app exercising the components. Build it with
`yew-tincture-test/build.sh`, which uses
[wasm-pack](https://rustwasm.github.io/wasm-pack/installer/), and serve the
resulting `pkg` directory with any static file server.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the commit message convention and the
release process.

## Code of conduct

Anyone who interacts with this software in any space, including but not limited
to this GitHub repository, must follow our [code of
conduct](code_of_conduct.md).

## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)
