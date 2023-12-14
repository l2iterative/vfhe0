use crate::N;

/// Represents an element of Z_{q}\[X\]/(X^N + 1) with implicit q = 2^64.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ResiduePoly {
    pub coefs: [u64; N],
}

impl ResiduePoly {
    pub fn add(&self, rhs: &ResiduePoly) -> Self {
        let mut res = Self::default();
        for i in 0..N {
            res.coefs[i] = self.coefs[i].wrapping_add(rhs.coefs[i]);
        }
        res
    }

    pub fn add_assign(&mut self, rhs: &ResiduePoly) {
        for i in 0..N {
            self.coefs[i] = self.coefs[i].wrapping_add(rhs.coefs[i]);
        }
    }

    pub fn sub(&self, rhs: &ResiduePoly) -> Self {
        let mut res = Self::default();
        for i in 0..N {
            res.coefs[i] = self.coefs[i].wrapping_sub(rhs.coefs[i]);
        }
        res
    }

    // TODO: use FFT for better performances
    pub fn mul(&self, rhs: &ResiduePoly) -> Self {
        let mut coefs = [0u64; N];
        for i in 0..N {
            let mut coef = 0u64;
            for j in 0..i + 1 {
                coef = coef.wrapping_add(self.coefs[j].wrapping_mul(rhs.coefs[i - j]));
            }
            for j in i + 1..N {
                coef = coef.wrapping_sub(self.coefs[j].wrapping_mul(rhs.coefs[N - j + i]));
            }
            coefs[i] = coef;
        }
        ResiduePoly { coefs }
    }

    /// Multiplies the residue polynomial by X^{exponent} = X^{2N + exponent}.
    /// `exponent` is assumed to be reduced modulo 2N.
    pub fn multiply_by_monomial(&self, exponent: usize) -> Self {
        let mut rotated_coefs = [0u64; N];

        let reverse = exponent >= N;
        let exponent = exponent % N;

        for i in 0..N {
            rotated_coefs[i] = {
                if i < exponent {
                    if reverse {
                        self.coefs[i + N - exponent]
                    } else {
                        self.coefs[i + N - exponent].wrapping_neg()
                    }
                } else if reverse {
                    self.coefs[i - exponent].wrapping_neg()
                } else {
                    self.coefs[i - exponent]
                }
            };
        }

        ResiduePoly {
            coefs: rotated_coefs,
        }
    }
}

impl Default for ResiduePoly {
    fn default() -> Self {
        ResiduePoly {
            coefs: [0u64; N],
        }
    }
}