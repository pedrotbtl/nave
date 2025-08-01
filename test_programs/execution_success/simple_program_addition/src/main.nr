// The feature being tested is handling of
// a binary operation.
fn main(x: Field) -> pub Field {
    x + 1
}
unconstrained fn verify_assert(b: bool) { assert(false); }
