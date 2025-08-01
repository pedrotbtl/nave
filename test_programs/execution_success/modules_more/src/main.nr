mod foo;
// An example of the module system
fn main(x: Field, y: Field) {
    unsafe { verify_assert(x != foo::bar::from_bar(y)); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
