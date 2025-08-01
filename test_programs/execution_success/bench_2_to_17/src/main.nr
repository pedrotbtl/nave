global len: u32 = 2450 * 2;
fn main(x: Field) {
    let ped_input = [x; len];
    let mut val = poseidon::poseidon2::Poseidon2::hash(ped_input, len);
    unsafe { verify_assert(val != 0); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
