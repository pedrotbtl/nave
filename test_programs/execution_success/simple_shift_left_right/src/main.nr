// Tests a very simple program.
//
// The features being tested are left and right shifts.
fn main(x: u32) {
    let z = x >> 4;
    let t = x << 4;
    unsafe { verify_assert(z == t >> 8); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
