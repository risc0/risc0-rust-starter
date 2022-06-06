# Building and Running the zkVM
*In this zkVM explainer, we take a look under the hood of the two `cargo run` commands from the [Quick Start instructions](README.md) of the `risc0-rust-starter` repo.*
### Building the zkVM's Methods

Before launching the zkVM, we build the required methods by running `$ cargo run --bin risc0-build-methods`. This instruction calls [`risc0_build::build_all()`](https://docs.rs/risc0-build/0.7.2/risc0_build/), which builds all RISC-V ELF binaries specified by `risc0 methods` metadata. 

### What Gets Built

After running the build command, you should see two new folders: `risc0-rust-starter/target` and `risc0-rust-starter/methods/guest/target`. The  former is responsible for launching the RISC Zero zkVM, and the latter contains the methods that will execute within the zkVM. 

Some noteworthy highlights:
- The Rust code in *multiply.rs* has been compiled to the RISC-V ELF file *risc0-rust-starter/methods/guest/target/riscv32im-unknown-none-elf/release/multiply*.
- This ELF file can be identified by its MethodID, which is the byte-by-byte contents of *target/release/build/methods-83db8ca8bfd5c024/out/multiply.id*
- The path of the ELF file and its MethodID can be called using the variable names `MULTPILY_PATH` and `MULTIPLY_ID`, as defined in *risc0-rust-starter/target/release/build/methods-83db8ca8bfd5c024/out/methods.rs*. 

The construction of the ELF file and the MethodID are implemented in the [risc0-build crate](https://docs.rs/risc0-build/0.7.2/src/risc0_build/lib.rs.html).



## Running the zkVM

With the methods in place, we launch the zkVM by running <br/>`$ cargo run --release --bin starter`

The [main.rs](https://github.com/risc0/risc0-rust-starter/blob/main/starter/src/main.rs) program loads the ELF file into the zkVM, executes it, and generates a [`receipt`](https://docs.rs/risc0-zkvm-host/0.7.2/risc0_zkvm_host/struct.Receipt.html) that serves as a zero-knowledge proof of computational integrity. 

## Paul's Open Questions 

- How is multiply.id generated? 

- main.rs relies on the [serialization/de-serialization crate](https://docs.rs/risc0-zkvm-serde/latest/risc0_zkvm_serde/) to convert between slices and vecs. What are slices and vecs? What do we mean by serialize and de-serialize?
  
- [risc0-rust-starter/methods/lib.rs](https://github.com/risc0/risc0-rust-starter/blob/main/methods/lib.rs) <br/>
  This is a 1-line program that reads `include!(concat!(env!("OUT_DIR"), "/methods.rs"));` What does this file do?

 - [risc0-rust-starter/methods/guest/build.rs](https://github.com/risc0/risc0-rust-starter/blob/main/methods/guest/build.rs) <br/>
    This is a 3-line program that simply calls [risc0_build::link()](https://docs.rs/risc0-build/0.7.2/risc0_build/), which does "special linking" for the zkVM. I'm assuming this means it links the ELF file to multiply.rs? 

- What is the "special entry point" of the ELF file, referenced in the risc0/risc0 README?

- What is the "special type of cryptographic hash of the ELF file," referenced in the risc0/risc0 README?





