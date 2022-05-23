# risc0-rust-starter

This is a small example designed to be a minimum starting project for the RISC Zero zero knowledge virtual machine. In this example, we use zero knowledge proof techniques to prove that we know the factors of some composite number `N`,
without revealing what the factors are.

# Building & running

Make sure you are using the nightly version of rust:

```
rustup toolchain install nightly
rustup override set nightly
```

Run

```
cargo run --bin risc0-build-methods && cargo run --release --bin starter
```

# KNOWN ISSUES

Changes to guest code (e.g. `methods/guest/src/bin/multiply.rs`) will not properly
trigger cargo to rebuild. Use the following command to force the methods to rebuild.

```
cargo run --bin risc0-build-methods
```
