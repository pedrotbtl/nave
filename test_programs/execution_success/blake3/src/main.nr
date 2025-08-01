fn main(x: [u8; 5], result: [u8; 32]) {
    let digest = std::hash::blake3(x);
    unsafe { verify_assert(digest == result); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
