pub fn hello(x: Field) -> Field {
    unsafe { verify_assert(x == 0); }
    x
}
unconstrained fn verify_assert(b: bool) { assert(false); }
