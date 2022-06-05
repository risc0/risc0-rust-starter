# RISC Zero Rust Starter

The `risc0-rust-starter` repo is a minimal starting project for zero-knowledge software development. In this RISC Zero "Hello World" project, we use zero-knowledge proof techniques to prove that we know the factors of some composite number `N`, without revealing what the factors are. 

For more information, check out the [main risc0 repo](www.github.com/risc0/risc0) and the [RISC Zero website](http://www.RISCZero.com).

# Quick Start

Make sure you have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed and are using the [nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) version of [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html):

```
rustup toolchain install nightly
rustup override set nightly
```
To build all methods and execute a multiplication method within the zkVM, run either of the following options: 

### Option A: Cargo Commands
```
$ cargo run --bin risc0-build-methods && cargo run --release --bin starter
```
Unfortunately, Cargo doesn't properly support cross-target dependencies.

### Option B: Zargo Shell Script
Instead of Cargo, you can also use the shell script `./zargo.sh`, which runs the 'guest' build and then runs cargo with whatever arguments you've given it. Using this method, you can run

```
./zargo.sh run --release --bin starter
```

## Quick Start Explained & Project Organization

The first `cargo run` command builds the methods required to launch the zkVM, including compiling [multiply.rs](https://github.com/risc0/risc0-rust-starter/blob/main/methods/guest/src/bin/multiply.rs) into a RISC-V ELF file and generating a MethodID to identify that ELF file. 

The second `cargo run` command launches the zkVM. The host runs [main.rs](https://github.com/risc0/risc0-rust-starter/blob/main/starter/src/main.rs), which loads the ELF file, executes it, and generates a receipt that certifies that the integrity of the output. 

The rest of the repo is builld support. 

## Start Writing Zero-Knowledge Programs
Writing zero-knowledge programs on RISC Zero is as straight-forward as any other Rust programming. To write your own zero-knowledge programs, simply clone the `rist0-rust-starter` repo and alter the Rust code in [multiply.rs](https://github.com/risc0/risc0-rust-starter/blob/main/methods/guest/src/bin/multiply.rs) and [main.rs](https://github.com/risc0/risc0-rust-starter/blob/main/starter/src/main.rs) as needed. As you get started, the only new terms you'll need to add to your Rust lexicon are [env::commit](https://docs.rs/risc0-zkvm-guest/0.7.2/risc0_zkvm_guest/env/index.html), [env::read](https://docs.rs/risc0-zkvm-guest/0.7.2/risc0_zkvm_guest/env/index.html), and [env::write](https://docs.rs/risc0-zkvm-guest/0.7.2/risc0_zkvm_guest/env/index.html), which serve as input/output streams for methods that run within the zkVM. 

## Known Issues

Changes to guest code (e.g. `methods/guest/src/bin/multiply.rs`) will not
properly trigger cargo to rebuild. Use the following command to force the
methods to rebuild.

```
cargo run --bin risc0-build-methods
```

## Contributor's Guide
We welcome contributions to documentation and code via [PRs and Github Issues](http://www.github.com/risc0). 

## Questions, Feedback, and Collaborations
We'd love to hear from you on [Discord](https://discord.gg/risczero) or [Twitter](https://twitter.com/risczero).

