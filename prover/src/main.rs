use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use methods::{MULTIPLY_ID, MULTIPLY_PATH};
use risc0_zkvm::{Prover};
use risc0_zkvm::serde::{from_slice, to_vec};

fn main() {
    // Pick two numbers.
    let a: u64 = 17;
    let b: u64 = 23;

    // First, we make the prover, loading the 'multiply' method.
    let multiply_src = std::fs::read(MULTIPLY_PATH)
        .expect("Method code should be present at the specified path.");
    let mut prover = Prover::new(&multiply_src, MULTIPLY_ID)
        .expect("Prover should be constructed.",);

    // Next we send a & b to the guest.
    prover.add_input_u32_slice(to_vec(&a).unwrap().as_slice());
    prover.add_input_u32_slice(to_vec(&b).unwrap().as_slice());
    
    // Run prover & generate receipt
    let receipt = prover.run()
        .expect("Valid code should be provable.");

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = from_slice(&receipt.journal)
        .expect("Journal output should deserialize.");

    // Print an assertion
    println!("I know the factors of {}, and I can prove it!", c);

    // Verify receipt, panic if it's wrong.
    receipt.verify(MULTIPLY_ID).expect(
        "Code you have proven should successfully verify.",
    );

    // Convert journal to string and store on disk.
    let journal = serde_json::to_string(&receipt.journal).unwrap();
    write_to_file("../data/journal", &journal);

    // Convert seal to string and store on disk.
    let seal = serde_json::to_string(&receipt.seal).unwrap();
    write_to_file("../data/seal", &seal);

    // Convert method_id to string and store on disk.
    let result = serde_json::to_string(MULTIPLY_ID).unwrap();
    write_to_file("../data/method", &result);
}

/// Writes a content to a file at the given path.
pub fn write_to_file(path: &str, content: &str) {
    let path = PathBuf::from(path);
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
