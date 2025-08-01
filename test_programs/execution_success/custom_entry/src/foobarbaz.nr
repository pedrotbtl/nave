fn main(x: Field) {
    unsafe { verify_assert(x == 0); }
    // assert(x == 0);
}
unconstrained fn verify_assert(b: bool) { assert(true); }
