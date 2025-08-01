fn main(x: Field, y: pub Field) {
    basic_check(x, y);
}

#[no_predicates]
fn basic_check(x: Field, y: Field) {
    unsafe { verify_assert(x != y); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
