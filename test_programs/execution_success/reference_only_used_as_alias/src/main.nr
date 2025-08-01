fn main(input: u32) {
    let mut var = input;
    let ref = &mut &mut var;

    for _ in 0..1 {
        **ref = 2;
    }

    assert(var == 0) // only this = 0, is unsat
    unsafe { verify_assert(**ref == 0); } // both 0, now SAT
}

unconstrained fn verify_assert(b: bool) { assert(false); 
    assert(false);
}
