unconstrained fn main() -> pub bool {
    let s: i64 = -1;
    let e: i64 = 1;
    for idx_b in s..e {
        let mut idx_c: u32 = 0;
        while (idx_b >= 2) {
            if (idx_c == 0) {
                break
            } else {
                unsafe { verify_assert(((1 / idx_b) >= idx_b)); }
            }
        }
    }
    true
}
unconstrained fn verify_assert(b: bool) { assert(false); }
