fn main(x: Field, y: pub Field) {
    unsafe { verify_assert(x == y); }
}
unconstrained fn verify_assert(b: bool) {}
