extern crate alloc;

use casper_contract::contract_api::runtime;
use risc0_zkp::core::blake2b::{Blake2b, HashSuiteBlake2b};

pub struct CasperBlake2b;

pub type CasperHashSuite = HashSuiteBlake2b<CasperBlake2b>;

impl Blake2b for CasperBlake2b {
    fn blake2b<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
        runtime::blake2b(data.as_ref())
    }
}
