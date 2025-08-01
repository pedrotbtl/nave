// Tests a very simple program.
//
// The features being tested is comparison
fn main(x: Field, y: Field) {
    unsafe { verify_assert(x as u32 < y as u32); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
