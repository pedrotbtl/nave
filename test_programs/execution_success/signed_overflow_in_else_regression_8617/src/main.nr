fn main(a: i32, c: bool) -> pub i32 {
    if c {
        a
    } else {
        ((-a) * a)
    }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
