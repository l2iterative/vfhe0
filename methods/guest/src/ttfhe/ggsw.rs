use crate::ttfhe::utils::round_value;
use crate::ttfhe::{glwe::GlweCiphertext, k, poly::ResiduePoly, ELL, N};

#[derive(Default, Clone, Copy)]
pub struct GgswCiphertext {
    pub z_m_gt: [GlweCiphertext; (k + 1) * ELL],
}

impl GgswCiphertext {
    /// Performs a product (GGSW x GLWE) -> GLWE.
    pub fn external_product(&self, ct: &GlweCiphertext) -> GlweCiphertext {
        let g_inverse_ct = apply_g_inverse(ct);

        let mut res = GlweCiphertext::default();
        for i in 0..(k + 1) * ELL {
            for j in 0..k {
                res.mask[j].add_assign(&g_inverse_ct[i].mul(&self.z_m_gt[i].mask[j]));
            }
            res.body
                .add_assign(&g_inverse_ct[i].mul(&self.z_m_gt[i].body));
        }
        res
    }
}

/// Decomposition of a GLWE ciphertext.
fn apply_g_inverse(ct: &GlweCiphertext) -> Vec<ResiduePoly> {
    let mut res: [ResiduePoly; (k + 1) * ELL] = Default::default();

    for i in 0..N {
        // mask decomposition
        for j in 0..k {
            let (nu_2, nu_1) = decomposition_8_2(ct.mask[j].coefs[i]);
            res[j * ELL].coefs[i] = nu_1 as u64;
            res[j * ELL + 1].coefs[i] = nu_2 as u64;
        }

        // body decomposition
        let (nu_2, nu_1) = decomposition_8_2(ct.body.coefs[i]);
        res[(k + 1) * ELL - 2].coefs[i] = nu_1 as u64;
        res[(k + 1) * ELL - 1].coefs[i] = nu_2 as u64;
    }
    res.to_vec()
}

/// Approximate decomposition with lg(B) = 8 and ell = 2.
/// Takes a polynomial coefficient in Z_{2^64} and decomposes its 16 MSBs in two signed 8-bit integers.
pub fn decomposition_8_2(val: u64) -> (i8, i8) {
    let rounded_val = round_value(val);
    if rounded_val & 128 == 128 {
        (rounded_val as i8, ((rounded_val >> 8) + 1) as i8)
    } else {
        (rounded_val as i8, (rounded_val >> 8) as i8)
    }
}

/// Ciphertext multiplexer. If `ctb` is an encryption of `1`, return `ct1`. Else, return `ct2`.
pub fn cmux(ctb: &GgswCiphertext, ct1: &GlweCiphertext, ct2: &GlweCiphertext) -> GlweCiphertext {
    let mut res = ct2.sub(ct1);
    res = ctb.external_product(&res);
    res = res.add(ct1);
    res
}
