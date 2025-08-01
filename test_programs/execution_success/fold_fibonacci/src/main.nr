fn main(x: u32) {
    unsafe { verify_assert(fibonacci(x) == 55); }
}

#[fold]
fn fibonacci(x: u32) -> u32 {
    if x <= 1 {
        x
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
