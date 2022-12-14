use odra::{Variable};
use risc0_zkvm::{Receipt};

// Import the proof and the method.
const METHOD_ID: &[u8] = &include!("../../data/method");
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
}

// The verification method. It constructs new Receipt and verifies it.
fn verify(journal: &[u32], seal: &[u32], method_id: &[u8]) -> String {
    let result = Receipt::new(&journal, &seal).verify(method_id);

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
