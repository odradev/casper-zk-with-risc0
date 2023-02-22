use odra::{Variable};
use blake2b::HashSuiteBlake2b;
use risc0_zkvm::{Receipt};

// Import the proof and the method.
const METHOD_ID: &[u32] = &include!("../../data/method");
const SEAL: &[u32] = &include!("../../data/seal");
const JOURNAL: &[u32] = &include!("../../data/journal");

// Verifier contract holds a result of the zk verfication. 
#[odra::module]
pub struct Verifier {
    result: Variable<String>,
}

#[odra::module]
impl Verifier {
    // Calling this entry point triggers the zk proof verification.
    pub fn verify(&mut self) {
        let result = verify(JOURNAL, SEAL, METHOD_ID);
        self.result.set(result);
    }

    // Result getter.   
    pub fn result(&self) -> String {
        self.result.get().unwrap_or(String::from("Not processed"))
    }

    pub fn verify_proof(&mut self, journal: Vec<u32>, seal: Vec<u32>) {
        let result = verify(&journal, &seal, METHOD_ID);
        self.result.set(result);
    }
}

// The verification method. It constructs new Receipt and verifies it.
fn verify(journal: &[u32], seal: &[u32], method_id: &[u32]) -> String {
    let method: [u32; 8] = [473203699,2556862419,3889962954,2376804667,2463669269,4258584453,4015235679,2598640211];
    let result = Receipt::new(&journal, &seal).verify_with_hash::<HashSuiteBlake2b, _>(&method);

    match result {
        Ok(()) => String::from("Ok"),
        Err(err) => format!("Error: {}", err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::VerifierDeployer;

    #[test]
    fn it_works() {
        let mut verifier = VerifierDeployer::default();
        verifier.verify();
        assert_eq!(verifier.result(), String::from("Ok"));
    }
}
