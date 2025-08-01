// Test integer addition: 3 + 4 = 7
fn main(mut x: u32) {
    unsafe { verify_assert(x > foo()); }
}

#[field(bn254)]
fn foo() -> u32 {
    1
}

#[field(23)]
fn foo() -> u32 {
    2
}

#[field(bls12_381)]
fn foo() -> u32 {
    3
}
unconstrained fn verify_assert(b: bool) { assert(false); }
