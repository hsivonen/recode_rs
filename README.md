# recode_rs

recode_rs is a test/sample app that's written in Rust and uses
[encoding_rs](https://github.com/hsivonen/encoding_rs).

## Building

On release-channel Rust:
```
cargo build --release
```

To enable SSE2 acceleration on nightly Rust:
```
cargo build --release --features simd-accel
```

## Licensing

Please see the file named COPYRIGHT.

