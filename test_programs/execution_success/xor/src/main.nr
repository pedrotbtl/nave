fn main(x: u32, y: pub u32) {
    let m = x ^ y;

    unsafe { verify_assert(m != 10); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
