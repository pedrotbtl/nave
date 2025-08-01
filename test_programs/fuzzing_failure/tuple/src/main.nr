// This program will fail when the input tuple is equal to the expected tuple
#[fuzz(only_fail_with = "broke_tuple")]
fn main(a: (u8, u8, u8, u8)) {
    let b = (0xde, 0xad, 0xbe, 0xef);
    // assert(a != b, "broke_tuple");
    unsafe { verify_assert(a != b); }
}

unconstrained fn verify_assert(b: bool) {}