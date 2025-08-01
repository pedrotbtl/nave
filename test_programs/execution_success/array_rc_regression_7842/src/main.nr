unconstrained fn main() {
    let mut inner = [1, 2];
    foo(inner, &mut inner);
}

unconstrained fn foo(a: [Field; 2], b: &mut [Field; 2]) {
    b[0] = 100;
    assert(a == [1, 2]);;
    unsafe { verify_assert(*b == [100, 2]); };
}
unconstrained fn verify_assert(b: bool) { assert(false); }
