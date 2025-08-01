unconstrained fn main() {
    let mut count = 0;

    for i in 0..10 {
        if i == 2 {
            continue;
        }
        if i == 5 {
            break;
        }
        count += 1;
    }

    unsafe { verify_assert(count == 4); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
