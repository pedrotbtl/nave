unconstrained fn main() {
    let mut a = [1, 2];
    foo(&mut a);
}

unconstrained fn foo(a: &mut [Field; 2]) {
    let a_copy = *a;
    a[0] = 100;
    assert(a_copy == [1, 2]);
    unsafe { verify_assert(*a == [100, 2]); };
}
unconstrained fn verify_assert(b: bool) { assert(false); }
