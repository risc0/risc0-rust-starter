use std::fs;

use methods::{MULTIPLY_ID, MULTIPLY_PATH};
use risc0_zkvm_host::Prover;
use risc0_zkvm_serde::{from_slice, to_vec};
use tempfile::tempdir;

fn main() {
    // Pick two numbers
    let a: u64 = 17;
    let b: u64 = 23;

    // Write the ID to a file, this is to work around the fact that the C++
    // prover API doesn't take IDs as buffers currently.
    let temp_dir = tempdir().unwrap();
    let id_path = temp_dir
        .path()
        .join("multiply.id")
        .to_str()
        .unwrap()
        .to_string();
    fs::write(&id_path, MULTIPLY_ID).unwrap();

    // Multiply them inside the ZKP
    // First, we make the prover, loading the 'multiply' method
    let mut prover = Prover::new(&MULTIPLY_PATH, &id_path).unwrap();
    // Next we send a + b to the guest
    prover.add_input(to_vec(&a).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&b).unwrap().as_slice()).unwrap();
    // Run prover + generate receipt
    let receipt = prover.run().unwrap();

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();

    // Print an assertation
    println!("I know the factors of {}, and I can prove it!", c);

    // Here is where one would send 'receipt' over the network...

    // Verify receipt, panic if it's wrong
    receipt.verify(&id_path).unwrap();
}
