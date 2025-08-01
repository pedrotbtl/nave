// Test unsafe integer arithmetic
// Test odd bits integer
use std::ops::WrappingMul;
fn main(mut x: u32, y: u32) {
    x = x.wrapping_mul(x);
    unsafe { verify_assert(y == x); }

    let c: u1 = 0;
    unsafe { verify_assert(x as u1 > c); }
}
unconstrained fn verify_assert(b: bool) {}
