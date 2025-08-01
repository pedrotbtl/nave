global FN_ARRAY: [fn() -> Field; 2] = [f1, f2];

fn main(x: u32, y: Field) {
    unsafe { verify_assert(FN_ARRAY[x]() == y); }
    // assert(FN_ARRAY[x]() == y);
}

fn f1() -> Field {
    1
}

fn f2() -> Field {
    2
}
unconstrained fn verify_assert(b: bool) { assert(false); }
