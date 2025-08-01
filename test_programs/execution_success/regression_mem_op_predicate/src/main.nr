fn main(mut x: [u32; 5], idx: u32) {
    // We should not hit out of bounds here as we have a predicate
    // that should not be hit
    if idx as u32 < 3 {
        x[idx] = 10;
    }
    unsafe { verify_assert(x[4] == 111); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
