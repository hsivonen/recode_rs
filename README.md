# recode_rs

[![crates.io](https://meritbadge.herokuapp.com/recode_rs)](https://crates.io/crates/recode_rs)
[![Apache 2 / MIT dual-licensed](https://img.shields.io/badge/license-Apache%202%20%2F%20MIT-blue.svg)](https://github.com/hsivonen/recode_rs/blob/master/COPYRIGHT)

recode_rs is a command-line tool converting between the character encodings
defined in the [Encoding Standard][1].

It is written primarily as sample code that demonstrates the use of
[encoding_rs][2], which is why it has an option for using UTF-16 (as opposed
to the default UTF-8) as the intermediate encoding, even though such an option
doesn't really make sense from the perspective of using the program as
non-sample code.

[1]: https://encoding.spec.whatwg.org/
[2]: https://github.com/hsivonen/encoding_rs

## Installing via `cargo`

Using release-channel Rust:
```
cargo install recode_rs
```

With SIMD acceleration on x86, x86_64 and Aarch64:
```
cargo install recode_rs --features simd-accel
```

## Building from a local git clone

Using release-channel Rust:
```
cargo build --release
```

With SIMD acceleration on x86, x86_64 and Aarch64:
```
cargo build --release --features simd-accel
```

## Usage

```
recode_rs [-f INPUT_ENCODING] [-t OUTPUT_ENCODING] [-o OUTFILE] [INFILE] [...]
```

### Options
```
    -o, --output PATH   set output file name (- for stdout; the default)
    -f, --from-code LABEL
                        set input encoding (defaults to UTF-8)
    -t, --to-code LABEL set output encoding (defaults to UTF-8)
    -u, --utf16-intermediate
                        use UTF-16 instead of UTF-8 as the intermediate
                        encoding
    -h, --help          print usage help
```

## Licensing

Please see the file named [COPYRIGHT][1].

[1]: https://github.com/hsivonen/recode_rs/blob/master/COPYRIGHT

## Release notes

### 1.0.6

* Use the `fast-legacy-encode` feature of encoding_rs 0.8.11.

### 1.0.5

* Update encoding_rs to 0.8.x.

### 1.0.4

* Tweak README.

### 1.0.3

* Initial crates.io release.
