# bdwgc-alloc

[![Circle CI](https://img.shields.io/circleci/project/github/raviqqe/bdwgc-alloc/master.svg?style=flat-square)](https://circleci.com/gh/raviqqe/bdwgc-alloc)
[![Crate](https://img.shields.io/crates/v/bdwgc-alloc.svg?style=flat-square)](https://crates.io/crates/bdwgc-alloc)
[![License](https://img.shields.io/github/license/raviqqe/bdwgc-alloc.svg?style=flat-square)](LICENSE)

[`GlobalAlloc`](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html) implementation for [`bdwgc`][bdwgc], the conservative garbage collector.

This crate is for use cases in which developers need to integrate [`bdwgc`][bdwgc] into their programs written in Rust (e.g. writing a runtime library in Rust for their own programming language whose GC is done by [`bdwgc`][bdwgc].)

## Usage

See [`examples`](examples) directory.

By default [`bdwgc`][bdwgc] is built with autotools. To build with cmake enable the `cmake` feature in `Cargo.toml`:

    [dependencies.bdwgc-alloc]
    version = "0.4"
    default-features = false
    features = ["cmake"]

## License

[MIT](LICENSE)

[bdwgc]: https://github.com/ivmai/bdwgc
