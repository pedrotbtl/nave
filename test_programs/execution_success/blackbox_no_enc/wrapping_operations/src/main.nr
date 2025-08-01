use std::ops::{WrappingAdd, WrappingMul, WrappingSub};

fn main(x: u8, y: u8) {
    unsafe { verify_assert(x.wrapping_sub(1) == y); }
    unsafe { verify_assert(y.wrapping_add(1) == x); }
    unsafe { verify_assert(y.wrapping_mul(y) == 1); }
}
unconstrained fn verify_assert(b: bool) {}
