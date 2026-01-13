fn main(x: Field, y: Field) {
    let p = x == y;
    unsafe { verify_assert(x == y); }
    unsafe { verify_assert(p == true); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
