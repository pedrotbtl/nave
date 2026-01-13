fn main(x: Field, y: Field) {
    let z = x == y;
    let t = z as u8;
    unsafe { verify_assert(z == true); }
    unsafe { verify_assert(t == 1); }
}

unconstrained fn verify_assert(b: bool) { assert(false); }
