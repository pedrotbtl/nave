fn main(mut x: i8, mut y: i8, z: i8) {
    let mut s1: i8 = 5;
    let mut s2: i8 = 8;
    assert(-(1 as i8) < 0);
    assert(x < y);
    assert(-x < y);
    assert(-y < -x);
    assert((z > x) == false);
    assert(x <= s1);
    unsafe { verify_assert(z < x - y - s2); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
