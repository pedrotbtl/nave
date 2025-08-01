pub struct Data {
    fields: [Field; 1],
    counter: u32,
}

fn main(array: call_data(0) [Data; 1], x: bool) {
    let index = if x { 0 } else { 1 };
    if index != 0 {
        unsafe { verify_assert(array[index - 1].counter < 3); }
    }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
