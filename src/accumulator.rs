use super::entropy_pool::EntropyPool;
use super::generator::Generator;
use rand_chacha::rand_core::TryRng;
use sha2::{Digest, Sha256};
use std::fmt;
use std::time::Instant;

const MIN_RESEED_INTERVAL: u128 = 100;
const MIN_POOL_SIZE: usize = 16; // 128 / 8 = 16

#[derive(Debug)]
pub enum AccumulatorError {
    NotSeeded,
}

impl fmt::Display for AccumulatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccumulatorError::NotSeeded => {
                write!(f, "Accumulator is not seeded!")
            }
        }
    }
}

impl std::error::Error for AccumulatorError {}

#[derive(Debug)]
pub struct Accumulator {
    pools: [EntropyPool; 32],
    generator: Generator,
    reseeds: u128,
    time: Instant,
}

impl Accumulator {
    pub fn new() -> Self {
        Accumulator {
            pools: [EntropyPool::new(); 32],
            generator: Generator::new(),
            reseeds: 0,
            time: Instant::now(),
        }
    }

    fn seed(&mut self) -> [u8; 32] {
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

    pub fn fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), AccumulatorError> {
        // Immedately seed if there is enough entropy.
        if !self.generator.seeded {
            if self.pools[0].size() >= MIN_POOL_SIZE {
                let seed = self.seed();
                self.generator.reseed(&seed);
                self.time = Instant::now();
            } else {
                return Err(AccumulatorError::NotSeeded);
            }
        }

        // Check if 100ms elapsed and if Pool 0 has enough bytes.
        if self.time.elapsed().as_millis() >= MIN_RESEED_INTERVAL
            && self.pools[0].size() >= MIN_POOL_SIZE
        {
            let seed = self.seed();
            self.generator.reseed(&seed);
            self.time = Instant::now();
        }

        self.generator.fill_bytes(dst);
        Ok(())
    }
}

impl TryRng for Accumulator {
    type Error = AccumulatorError;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        let mut buffer = [0u8; 4];
        self.fill_bytes(&mut buffer)?;

        Ok(u32::from_le_bytes(buffer))
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let mut buffer = [0u8; 8];
        self.fill_bytes(&mut buffer)?;

        Ok(u64::from_le_bytes(buffer))
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        self.fill_bytes(dst)
    }
}
