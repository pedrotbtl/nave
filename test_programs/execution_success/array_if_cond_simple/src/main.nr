fn main(x: bool, mut y: [u32; 30]) {
    if x {
        y[0] = 1;
    }

    let z = y[0] + y[1];
    unsafe { verify_assert(z == 1); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
