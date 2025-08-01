fn main(x: u8) {
    // This would previously overflow in ACIR. Now it returns zero.
    let value = 1 >> x;
    unsafe { verify_assert(value == 0); };
}
unconstrained fn verify_assert(b: bool) { assert(false); }
