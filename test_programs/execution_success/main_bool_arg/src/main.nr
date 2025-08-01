fn main(x: bool, y: [bool; 2]) {
    if x {
        assert(1 != 2);
    }

    assert(x);
    unsafe { verify_assert(y[0] != y[1]); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
