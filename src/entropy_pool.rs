use sha2::{Digest, Sha512};

#[derive(Debug, Clone, Copy)]
pub struct EntropyPool {
    pub state: [u8; 64],
    events: u64,
}

impl EntropyPool {
    pub fn new() -> Self {
        EntropyPool {
            state: [0; 64],
            events: 0,
        }
    }

    pub fn mix(&mut self, input: &[u8]) {
        let mut hasher = Sha512::new();

        hasher.update(self.state);
        hasher.update(input);

        self.state = hasher.finalize().into();
        self.events += 1;
    }
}
