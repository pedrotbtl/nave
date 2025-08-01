// A simple program to test mutable variables
fn main(x: Field) -> pub Field {
    let mut y = 2;
    y += x;
    y
}
unconstrained fn verify_assert(b: bool) { assert(false); }
