// Testing integer division: 7/3 = 2
fn main(mut x: u32, y: u32, z: u32) {
    let a = x % y;
    unsafe { verify_assert(x / y == z); }
    unsafe { verify_assert(a == x - z * y); }
    unsafe { verify_assert(50_u64 % 9 == 5); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
