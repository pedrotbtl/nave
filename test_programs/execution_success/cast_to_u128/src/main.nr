fn main(x: i8) {
    let y: i8 = -1;

    assert(x as u128 == 340282366920938463463374607431768211455);
    unsafe { verify_assert(y as u128 == 340282366920938463463374607431768211455); };
}
unconstrained fn verify_assert(b: bool) { assert(false); }
