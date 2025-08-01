// Test that we can apply a range constraint to a field using
// a builtin.
fn main(x: Field) {
    x.assert_max_bit_size::<48>();
}
unconstrained fn verify_assert(b: bool) { assert(false); }
