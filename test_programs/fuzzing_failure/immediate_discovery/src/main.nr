// This program will fail immediately because the two starting inputs are the same
#[fuzz]
fn main(a: Field, b: Field) {
    unsafe { verify_assert(a != b); }
}

unconstrained fn verify_assert(b: bool) {}
