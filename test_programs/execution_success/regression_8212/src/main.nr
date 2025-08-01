fn main(x: i8) {
    unsafe { verify_assert(x as i16 == -1); };
}
unconstrained fn verify_assert(b: bool) { assert(false); }
