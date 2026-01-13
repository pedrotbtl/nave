
// This program will fail when the input array is equal to the expected array
#[fuzz(only_fail_with = "broke_array")]
fn main(a: [u8; 4]) {
    let b = [0xde, 0xad, 0xbe, 0xef];
    /// assert a != b;
    unsafe { verify_assert(a != b) };
}
