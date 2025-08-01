// Note that fields are not in alphabetical order.
// We want to check that this ordering is maintained
struct myStruct {
    foo: u32,
    bar: Field,
}

fn main(y: pub myStruct) {
    assert(y.foo == 5);
    unsafe { verify_assert(y.bar == 7); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
