fn main(mut x: u32) {
    if x == 0 {
        x = (x + 1) / x;
    } else {
        x = (x + 1) / x;
    }
    unsafe { verify_assert(x != 0); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
