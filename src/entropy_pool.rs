use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy)]
pub struct EntropyPool {
    state: [u8; 32],
}

impl EntropyPool {
    pub fn new() -> Self {
        EntropyPool { state: [0; 32] }
    }

    pub fn mix(&mut self, input: &[u8]) {
        let mut hasher = Sha256::new();

        hasher.update(self.state);
        hasher.update(input);

        self.state = hasher.finalize().into();
    }

    pub fn take_state(&mut self) -> [u8; 32] {
        let current_state = self.state;
        self.state = [0; 32];
        current_state
    }
}
