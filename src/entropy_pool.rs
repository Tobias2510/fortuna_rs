use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy)]
pub struct EntropyPool {
    state: [u8; 32],
    count: usize,
}

impl EntropyPool {
    pub fn new() -> Self {
        EntropyPool {
            state: [0; 32],
            count: 0,
        }
    }

    pub fn mix(&mut self, input: &str) { // Not sure yet if I should use &str or &[u8].
        let mut hasher = Sha256::new();

        hasher.update(self.state);
        hasher.update(input);

        self.state = hasher.finalize().into();
        self.count+= 1;
    }

    pub fn size(&self) -> usize {
        self.count
    }

    pub fn take_state(&mut self) -> [u8; 32] {
        let current_state = self.state;

        self.state = [0; 32];
        self.count = 0;

        current_state
    }
}
