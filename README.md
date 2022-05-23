# risc0-rust-starter

This is a small example designed to be a minimum starting project for the RISC Zero zero knowledge virtual machine.
In this example, we use zero knowledge proof techniques to prove that we know the factors of some composite number `N`, without revealing what the factors are.

For the main project, see [here](https://github.com/risc0/risc0)

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

The first command compiles the 'guest' code, which runs inside the zero knowledge VM, and the second compiles the 'host' code that runs the ZKVM.
Unfortunately, cargo doesn't properly support cross-target dependencies.

Alternately, we've included a shell script `./zargo.sh`, which runs the 'guest' build and then runs cargo with whatever arguments you've given it, so you can instead run

```
./zargo.sh run --release --bin starter
````

# Project organization

The main program, which invokes the ZKP system and calls a method in the ZKVM is in [starter/src/main.rs](starter/src/main.rs).
The code that runs inside the ZKVM is in [methods/guest/src/bin/multiply.rs](methods/guest/src/bin/multiply.rs).
The rest of the project is build support.

# KNOWN ISSUES

Changes to guest code (e.g. `methods/guest/src/bin/multiply.rs`) will not
properly trigger cargo to rebuild. Use the following command to force the
methods to rebuild.

```
cargo run --bin risc0-build-methods
```
