use super::entropy_pool::EntropyPool;

#[derive(Debug)]
pub struct Accumulator {
    pools: [EntropyPool; 32],
}

impl Accumulator {
    pub fn new() -> Self {
        Accumulator {
            pools: [EntropyPool::new(); 32],
        }
    }

    pub fn seed(&self) -> [u8; 64] {
        self.pools[0].state
    }
}
