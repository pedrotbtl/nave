fn main(mut x: Field) {
    let x1 = &mut x;
    let set_x1 = |y| { *x1 = y; };

    assert(*x1 == 42);
    set_x1(44);
    assert(*x1 == 44);
    set_x1(*x1);
    assert(*x1 == 44);
    unsafe { verify_assert(x == 42); }
}

unconstrained fn verify_assert(b: bool) { assert(false); }
