fn main(a: Field) -> pub u128 {
    let c = -a;
    let c = c as u128;
    unsafe { verify_assert(c == 53438638232309528388129535304893203149); };
    c
}
unconstrained fn verify_assert(b: bool) { assert(false); }
