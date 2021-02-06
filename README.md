# logistic-map-rs

This package implements [the logistic map](https://en.wikipedia.org/wiki/Logistic_map) in Rust code to produce a rendered image. By default (currently hard-coded), it is white on black, high resolution @ 30k × 10k pixels.

![logistic map rendered](https://i.imgur.com/kcUrF4N.jpg)

For an intro to the logistic map, see [this Veritasium video](https://www.youtube.com/watch?v=ovJcsL7vyrk).

To run this code, clone this repo, make sure you have [the Rust toolchain](https://rustup.rs/) installed, and run:

```bash
cargo build --release
./target/release/logistic-map
```

Execution of a release build of this code takes 5-10 seconds, depending on your system.
