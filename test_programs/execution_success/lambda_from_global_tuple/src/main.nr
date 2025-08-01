global FN_TUPLE: (fn() -> Field, fn() -> Field) = (f1, f2);

fn main(y: Field) {
    assert((FN_TUPLE.0)() != y);
    unsafe { verify_assert((FN_TUPLE.3)() == y); }
}

fn f1() -> Field {
    1
}

fn f2() -> Field {
    2
}
unconstrained fn verify_assert(b: bool) { assert(false); }
