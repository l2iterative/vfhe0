mod cycle_trace;

use core::hint::black_box;
use ttfhe::{N,
            ggsw::{cmux, GgswCiphertext},
            glwe::GlweCiphertext,
            lwe::LweCiphertext
};
risc0_zkvm::guest::entry!(main);

static BSK_BYTES: &[u8] = include_bytes!("../../../bsk");
static C_BYTES: &[u8] = include_bytes!("../../../c");

pub fn main() {
    cycle_trace::init_trace_logger();
    start_timer!("Total");

    start_timer!("Load the bootstrapping key");

    let bsk = black_box(unsafe {
        std::mem::transmute::<&u8, &[GgswCiphertext; 16]>(&BSK_BYTES[0])
    });

    stop_start_timer!("Load the ciphertext to be bootstrapped");

    let c = black_box(unsafe {
        std::mem::transmute::<&u8, &LweCiphertext>(&C_BYTES[0])
    });

    stop_start_timer!("Perform trivial encryption and rotation");

    let lut = black_box(GlweCiphertext::trivial_encrypt_lut_poly());
    let mut c_prime = lut.clone();
    c_prime.rotate_trivial((2 * N as u64) - c.body);

    stop_start_timer!("Perform one step of the bootstrapping");

    for i in 0..1 {
        start_timer!("Rotate");
        let rotated = c_prime.rotate(c.mask[i]);

        stop_start_timer!("Cmux");
        c_prime = cmux(&bsk[i], &c_prime, &rotated);

        stop_timer!();
    }

    stop_timer!();

    stop_timer!();
}