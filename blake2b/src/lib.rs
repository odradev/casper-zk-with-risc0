extern crate alloc;

use risc0_core::field::baby_bear::{BabyBear, BabyBearElem, BabyBearExtElem};
use risc0_zkp::core::config::{ConfigHash, ConfigRng};
use risc0_zkp::core::config::HashSuite;
use risc0_zkvm::{ControlIdLocator, ControlId, POSEIDON_CONTROL_ID};
use alloc::boxed::Box;
use risc0_zkp::core::digest::Digest;
use risc0_core::field::ExtElem;
use alloc::vec::Vec;
use casper_contract::contract_api::runtime;


/// HashSuite
pub struct HashSuiteBlake2b {}

impl HashSuite<BabyBear> for HashSuiteBlake2b {
    type Hash = ConfigHashBlake2b;
    type Rng = Blake2bRng;
}

/// ConfigHash
pub struct ConfigHashBlake2b {}

impl ConfigHash<BabyBear> for ConfigHashBlake2b {
    type DigestPtr = Box<Digest>;

    fn hash_pair(a: &Digest, b: &Digest) -> Self::DigestPtr {
        let concat = [a.as_bytes().as_ref(), b.as_bytes()].concat();
        Box::new(Digest::from(blake2b(concat)))
    }

    fn hash_elem_slice(slice: &[BabyBearElem]) -> Self::DigestPtr {
        let mut data = Vec::<u8>::new();
        for el in slice {
            data.extend_from_slice(el.as_u32_montgomery().to_be_bytes().as_slice());
        }
        Box::new(Digest::from(blake2b(data)))
    }

    fn hash_ext_elem_slice(slice: &[BabyBearExtElem]) -> Self::DigestPtr {
        let mut data = Vec::<u8>::new();
        for ext_el in slice {
            for el in ext_el.subelems() {
                data.extend_from_slice(el.as_u32_montgomery().to_be_bytes().as_slice());
            }
        }
        Box::new(Digest::from(blake2b(data)))
    }
}

/// A random number generator.
pub struct Blake2bRng {
    current: [u8; 32]
}

impl ConfigRng<BabyBear> for Blake2bRng {
    fn new() -> Self {
        Self {
            current: [0; 32],
        }
    }

    fn mix(&mut self, val: &Digest) {
        let concat = [self.current.as_ref(), val.as_bytes()].concat();
        self.current = blake2b(concat);
    }

    fn random_u32(&mut self) -> u32 {
        let next = blake2b(self.current);
        self.current = next;

        ((next[0] as u32) << 24) +
            ((next[1] as u32) << 16) +
            ((next[2] as u32) <<  8) +
            ((next[3] as u32) <<  0)
    }

    fn random_elem(&mut self) -> BabyBearElem {
        BabyBearElem::new(self.random_u32())
    }

    fn random_ext_elem(&mut self) -> BabyBearExtElem {
        BabyBearExtElem::new(
            self.random_elem(),
            self.random_elem(),
            self.random_elem(),
            self.random_elem()
        )
    }
}

fn blake2b<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    runtime::blake2b(data.as_ref())
}

impl ControlIdLocator for ConfigHashBlake2b {
    fn get_control_id() -> ControlId {
        // TODO: Implement for blake2b
        let mut table = alloc::vec::Vec::new();
        for entry in POSEIDON_CONTROL_ID {
            //table.push(Digest::from_hex(entry).unwrap());
        }
        ControlId { table }
    }
}