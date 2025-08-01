fn main(x: Field, active: bool) {
    let mut x1 = false;
    // Safety:)
    unsafe {
        if active {
            x1 = foo(x);
        }

        let x2 = foo(x);
        unsafe { verify_assert(x2); }
    }
}

unconstrained fn foo(_x: Field) -> bool {
    true
}
unconstrained fn verify_assert(b: bool) { assert(false); }
