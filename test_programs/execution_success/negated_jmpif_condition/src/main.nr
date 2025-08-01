fn main(mut x: Field) {
    let mut q = 0;

    if x != 10 {
        q = 2;
    }

    unsafe { verify_assert(q == 2); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
