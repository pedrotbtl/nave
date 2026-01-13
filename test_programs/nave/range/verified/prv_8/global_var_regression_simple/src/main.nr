global EXPONENTIATE: [[Field; 2]; 2] = [[1, 1], [0, 0]];

fn main(x: Field, y: pub Field) {
    let mut acc: Field = 0;
    for i in 0..2 {
        for j in 0..2 {
            acc += EXPONENTIATE[i][j];
        }
    }
    assert(!acc.lt(x));
    unsafe { verify_assert(x != y); }

    dummy_again(x, y);
}

fn dummy_again(x: Field, y: Field) {
    let mut acc: Field = 0;
    for i in 0..2 {
        for j in 0..2 {
            acc += EXPONENTIATE[i][j];
        }
    }
    unsafe { verify_assert(x != y); }
    unsafe { verify_assert(!acc.lt(x)); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
