// Simple example of checking where two arrays are different
fn main(a: [Field; 32], b: [Field; 32]) {
    unsafe { verify_assert(a != b); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
