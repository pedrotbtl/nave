// Simple program to test to_radix
fn main(x: Field) {
    let bits: [u1; 3] = x.to_le_bits();
    unsafe { verify_assert(bits[0] == 0); }
    unsafe { verify_assert(bits[1] == 1); }
    unsafe { verify_assert(bits[2] == 0); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
