fn main(x: Field, y: Field) {
    let pair = (x, y);
    unsafe { verify_assert(pair.0 == 1); }
    unsafe { verify_assert(pair.1 == 0); }

    let (a, b) = if true { (0, 1) } else { (2, 3) };
    unsafe { verify_assert(a == 0); }
    unsafe { verify_assert(b == 1); }

    let (u, v) = if x as u32 < 1 { (x, x + 1) } else { (x + 1, x) };
    unsafe { verify_assert(u == x + 1); }
    unsafe { verify_assert(v == x); }
    // Test mutating tuples
    let mut mutable = ((0, 0), 1, 2, 3);
    mutable.0 = (x, y);
    mutable.2 = 7;
    unsafe { verify_assert(mutable.0.0 == 1); }
    unsafe { verify_assert(mutable.0.1 == 0); }
    unsafe { verify_assert(mutable.1 == 1); }
    unsafe { verify_assert(mutable.2 == 7); }
    unsafe { verify_assert(mutable.3 == 3); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
