# mikktspace-sys

[![crates.io](https://img.shields.io/crates/v/mikktspace-sys.svg)](https://crates.io/crates/mikktspace-sys)

Wrapper around the original C reference implementation ([Mikkelsen Tangent Space Algorithm](https://en.blender.org/index.php/Dev:Shading/Tangent_Space_Normal_Maps)).

The reason for this crate is that for practical applications the output must match the C reference implementation exactly. The Rust mikktspace crate is probably producing identical output in all cases, but I didn't want to take any chances.

## Examples

### generate

Demonstrates generating tangents for a cube with 4 triangular faces per side.

```sh
cargo run --example generate
```

## License agreement

 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
