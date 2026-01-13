// Tests a very simple program.
//
// The features being tested is assertion
fn main(x: Field, y: pub Field) {
    unsafe { verify_assert(x == y); }
    unsafe { verify_assert(x == y); };
}
unconstrained fn verify_assert(b: bool) { assert(false); }
