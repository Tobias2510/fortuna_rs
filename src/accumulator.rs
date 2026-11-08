use super::entropy_pool::EntropyPool;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Accumulator {
    pools: [EntropyPool; 32],
    reseeds: u128,
}

impl Accumulator {
    pub fn new() -> Self {
        Accumulator {
            pools: [EntropyPool::new(); 32],
            reseeds: 0,
        }
    }

    pub fn seed(&mut self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        self.reseeds += 1;

        for index in 0..32 {
            let base: u128 = 2;
            if self.reseeds % base.pow(index as u32) == 0 {
                let state = self.pools[index].take_state();
                hasher.update(state);
            }
        }

        hasher.finalize().into()
    }
}
