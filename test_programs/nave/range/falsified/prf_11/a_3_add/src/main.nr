// Test integer addition: 3 + 4 = 7
fn main(mut x: u32, y: u32, z: u32) {
    x += y;
    unsafe { verify_assert(x == z); }

    x *= 8;
    unsafe { verify_assert(x > 9); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
