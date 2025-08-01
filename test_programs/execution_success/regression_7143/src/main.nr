fn main(x: u32, array: call_data(0) [bool; 1]) -> pub bool {
    !array[x]
}
unconstrained fn verify_assert(b: bool) { assert(false); }
