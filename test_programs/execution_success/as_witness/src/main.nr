// Simple example of checking where two arrays are different
fn main(a: Field) -> pub Field {
    std::as_witness(a);
    a
}
unconstrained fn verify_assert(b: bool) { assert(false); }
