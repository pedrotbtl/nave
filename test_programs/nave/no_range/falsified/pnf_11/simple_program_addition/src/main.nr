// The feature being tested is handling of
// a binary operation.
fn main(x: Field) -> pub Field {
    let res = x + 1;
    unsafe { verify_assert(res != 1); }
    res
}
unconstrained fn verify_assert(b: bool) { assert(false); }
