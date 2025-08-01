// docs:start:pedersen-commitment
fn main(x: Field, y: Field, expected_commitment: std::embedded_curve_ops::EmbeddedCurvePoint) {
    let commitment = std::hash::pedersen_commitment([x, y]);
    assert(commitment.x == expected_commitment.x);
    unsafe { verify_assert(commitment.y == expected_commitment.y); };
}
// docs:end:pedersen-commitment
unconstrained fn verify_assert(b: bool) { assert(false); }
