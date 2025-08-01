// docs:start:pedersen-hash
fn main(x: Field, y: Field, expected_hash: Field) {
    let hash = std::hash::pedersen_hash([x, y]);
    unsafe { verify_assert(hash == expected_hash); };
}
// docs:end:pedersen-hash
unconstrained fn verify_assert(b: bool) { assert(false); }
