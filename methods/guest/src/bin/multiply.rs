#![no_main]
#![no_std]

use risc0_zkvm_guest::env;

risc0_zkvm_guest::entry!(main);

pub fn main() {
    // Load the first number from the host
    let a: u64 = env::read();
    // Load the second number from the host
    let b: u64 = env::read();
    // Verify that neither of them are 1 (i.e. nontrivial factors)
    if a == 1 || b == 1 {
        panic!("Trivial factors")
    }
    // Compute the product
    let c: u64 = a * b;
    // Commit it to the public journal
    env::commit(&c);
}
