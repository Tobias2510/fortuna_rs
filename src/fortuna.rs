use super::entropy_pool::EntropyPool;

#[derive(Debug)]
pub struct Fortuna {
    pools: [EntropyPool; 32],
}

impl Fortuna {
    pub fn new() -> Self {
        Fortuna {
            pools: [EntropyPool::new(); 32],
        }
    }
}
