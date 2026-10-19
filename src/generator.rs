use rand_chacha::{
    ChaCha20Rng,
    rand_core::{Rng, SeedableRng, TryCryptoRng, TryRng},
};
use sha2::{Digest, Sha256};
use std::convert::Infallible;

#[derive(Debug)]
pub struct Generator {
    rng: ChaCha20Rng,
    pub seeded: bool,
}

const MAX_REQUEST_BYTES: usize = 1 << 20; // 2^20 -> 1Mb

impl Generator {
    pub fn new() -> Self {
        let init_seed = Default::default();
        let rng = ChaCha20Rng::from_seed(init_seed);

        Generator { rng, seeded: false }
    }

    pub fn reseed(&mut self, input: &[u8]) {
        let mut hasher = Sha256::new();
        let old_seed = self.rng.get_seed();

        hasher.update(old_seed);
        hasher.update(input);

        let seed: [u8; 32] = hasher.finalize().into();

        let pos = self.rng.get_word_pos();
        self.rng = ChaCha20Rng::from_seed(seed);
        self.rng.set_word_pos(pos + 16);

        self.seeded = true;
    }

    pub fn fill_bytes(&mut self, dst: &mut [u8]) {
        if !self.seeded {
            panic!("Generator is not seeded yet!");
        }

        if dst.len() > MAX_REQUEST_BYTES {
            panic!("Not more data than 1Mb allowed!")
        }

        match self.rng.try_fill_bytes(dst) {
            Ok(()) => (),
        }

        // Rekey with the next 256 bits of the CSPRNG
        let mut new_key = [0u8; 32];
        self.rng.fill_bytes(&mut new_key);

        let pos = self.rng.get_word_pos();
        self.rng = ChaCha20Rng::from_seed(new_key);
        self.rng.set_word_pos(pos);
    }
}

impl TryRng for Generator {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Infallible> {
        let mut buffer = [0u8; 4];
        self.fill_bytes(&mut buffer);

        Ok(u32::from_le_bytes(buffer))
    }

    fn try_next_u64(&mut self) -> Result<u64, Infallible> {
        let mut buffer = [0u8; 8];
        self.fill_bytes(&mut buffer);

        Ok(u64::from_le_bytes(buffer))
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Infallible> {
        self.fill_bytes(dst);
        Ok(())
    }
}

impl SeedableRng for Generator {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        let rng = ChaCha20Rng::from_seed(seed);
        Generator { rng, seeded: true }
    }
}

impl TryCryptoRng for Generator {}
