fn main(x: u1, y: u1) {
    unsafe { verify_assert(x | y == 1); }

    unsafe { verify_assert(x | y | x == 1); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
