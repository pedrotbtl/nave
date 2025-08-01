fn main(x: u1) {
    unsafe { verify_assert(!x == 0); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
