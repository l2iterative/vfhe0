use crate::ttfhe::LWE_DIM;

#[derive(Clone)]
pub struct LweCiphertext {
    pub mask: [u64; LWE_DIM],
    pub body: u64,
}

impl Default for LweCiphertext {
    fn default() -> Self {
        LweCiphertext {
            mask: [0u64; LWE_DIM],
            body: 0u64,
        }
    }
}