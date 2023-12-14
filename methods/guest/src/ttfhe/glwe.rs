use crate::ttfhe::utils::encode;
use crate::ttfhe::P;
use crate::ttfhe::{k, poly::ResiduePoly, N};

#[derive(Clone, Copy, Default)]
pub struct GlweCiphertext {
    pub mask: [ResiduePoly; k],
    pub body: ResiduePoly,
}

/// Set of `k` polynomials in {0, 1}\[X\]/(X^N + 1).
#[derive(Clone)]
#[repr(C)]
pub struct SecretKey {
    pub polys: [ResiduePoly; k],
}

impl GlweCiphertext {
    pub fn add(&self, rhs: &Self) -> Self {
        let mut res = GlweCiphertext::default();
        for i in 0..k {
            res.mask[i] = self.mask[i].add(&rhs.mask[i]);
        }
        res.body = self.body.add(&rhs.body);
        res
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        let mut res = GlweCiphertext::default();
        for i in 0..k {
            res.mask[i] = self.mask[i].sub(&rhs.mask[i]);
        }
        res.body = self.body.sub(&rhs.body);
        res
    }

    /// Multiplies by the monomial `X^exponent` the body of `self`.
    /// `self` is assumed to be a trivial encryption.
    pub fn rotate_trivial(&mut self, exponent: u64) {
        self.body = self.body.multiply_by_monomial(exponent as usize);
    }

    /// Multiplies by the monomial `X^exponent` every component of `self`.
    pub fn rotate(&self, exponent: u64) -> Self {
        let mut res = Self::default();

        start_timer!("handle the mask");
        for i in 0..k {
            res.mask[i] = self.mask[i].multiply_by_monomial(exponent as usize);
        }
        stop_start_timer!("handle the body");

        res.body = self.body.multiply_by_monomial(exponent as usize);
        stop_timer!();

        res
    }

    /// Trivially encrypts the LUT polynomial.
    pub fn trivial_encrypt_lut_poly() -> Self {
        let mut lut_coefs = [0u64; N];

        for i in 0..N {
            lut_coefs[(i.wrapping_sub(64)) % N] = encode(((P * i) / (2 * N)).try_into().unwrap());
        }

        Self {
            body: ResiduePoly { coefs: lut_coefs },
            ..Default::default()
        }
    }
}