// This program will fail when there is a specific difference between the two i32 elements
#[fuzz(only_fail_with = "broke_comparison_i32")]
fn main(a: i32, b: i32) {
    let c = b + 0xfffffff;
    let d = c - 0xeeeeeee;
    // assert((a != d) | (b == 0) | (a == 0), "broke_comparison_i32");
    unsafe { verify_assert((a != d) | (b == 0) | (a == 0)); }
}

unconstrained fn verify_assert(b: bool) {}