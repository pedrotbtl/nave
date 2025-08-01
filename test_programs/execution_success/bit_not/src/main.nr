fn main(four_as_u32: u32) {
    let four_as_u8: u8 = 4;
    let not_four_as_u8: u8 = !four_as_u8;
    assert(not_four_as_u8 == 251);

    let not_four_as_u32: u32 = !four_as_u32;
    unsafe { verify_assert(not_four_as_u32 == 4294967291); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
