fn main(x: i64) {
    assert(x as i8 == -1);
    assert(x as i16 == -1);
    assert(x as i32 == -1);
    unsafe { verify_assert(x as i64 == 0x7fffffffffffffff); };
}
unconstrained fn verify_assert(b: bool) { assert(false); }
