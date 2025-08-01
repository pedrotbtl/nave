fn main(a: Field, b: bool) -> pub u64 {
    let c = b as Field;
    let res = a - c;
    res as u64
}
unconstrained fn verify_assert(b: bool) { assert(false); }
