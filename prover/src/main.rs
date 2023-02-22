use methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_circuit_rv32im::cpu::CpuEvalCheck;
use risc0_core::field::baby_bear::BabyBear;
use risc0_zkp::core::blake2b::{Blake2bCpuImpl, HashSuiteBlake2b};
use risc0_zkp::hal::cpu::CpuHal;
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::{Prover, ProverOpts};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Pick two numbers.
    let a: u64 = 17;
    let b: u64 = 23;

    // Multiply them inside the ZKP
    // First, we make the prover, loading the 'multiply' method
    let opts = ProverOpts::default().with_skip_verify(true);
    let mut prover = Prover::new_with_opts(MULTIPLY_ELF, MULTIPLY_ID, opts).expect(
        "Prover should be constructed from valid method source code and corresponding method ID",
    );

    // Next we send a & b to the guest
    prover.add_input_u32_slice(&to_vec(&a).expect("should be serializable"));
    prover.add_input_u32_slice(&to_vec(&b).expect("should be serializable"));

    let hal = CpuHal::<BabyBear, HashSuiteBlake2b<Blake2bCpuImpl>>::new();
    let circuit = risc0_circuit_rv32im::CircuitImpl::new();
    let eval = CpuEvalCheck::new(&circuit);
    // Run prover & generate receipt
    let receipt = prover
        .run_with_hal(&hal, &eval)
        .expect("Should be able to prove valid code that fits in the cycle count.");

    // Convert journal to string and store on disk.
    let journal = serde_json::to_string(&receipt.journal).unwrap();
    write_to_file("../data/journal", &journal);

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = from_slice(&receipt.journal).expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    // Print an assertion
    println!("I know the factors of {}, and I can prove it!", c);

    // Here is where one would send 'receipt' over the network...

    // Verify receipt, panic if it's wrong
    receipt.verify_with_hash::<HashSuiteBlake2b<Blake2bCpuImpl>, _>(&MULTIPLY_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );

    // Convert journal to string and store on disk.
    let journal = serde_json::to_string(&receipt.journal).unwrap();
    write_to_file("../data/journal", &journal);

    // Convert seal to string and store on disk.
    let seal = serde_json::to_string(&receipt.seal).unwrap();
    write_to_file("../data/seal", &seal);

    // Convert method_id to string and store on disk.
    let result = serde_json::to_string(&MULTIPLY_ID).unwrap();
    write_to_file("../data/method", &result);
}

/// Writes a content to a file at the given path.
pub fn write_to_file(path: &str, content: &str) {
    let path = PathBuf::from(path);
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
