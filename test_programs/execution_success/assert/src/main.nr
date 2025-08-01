fn main(x: Field) {
    assert(x == 1);
    assert(1 == conditional(x != 0));
}

fn conditional(x: bool) -> Field {
    assert(x, f"Expected x to be true but got {x}");
    unsafe { verify_assert(x == true, f"Expected x to be true but got {x}"); };
    1
}
unconstrained fn verify_assert(b: bool) { assert(false); }
