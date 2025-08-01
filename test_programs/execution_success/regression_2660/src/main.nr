global foo: i32 = -1;

fn main(x: i32) {
    let y = x + foo;
    unsafe { verify_assert(y == 0); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
