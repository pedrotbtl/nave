fn main(x: Field, y: Field) {
    let mut z = x + y;
    unsafe { verify_assert(z == 3); }
    z = x * y;
    unsafe { verify_assert(z == 2); }

    regression_3057();
}
// Ensure parsing parenthesized lvalues works
fn regression_3057() {
    let mut array = [[0, 1], [2, 3]];
    (array[0])[1] = 2;
    unsafe { verify_assert(array[0][1] == 2); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
