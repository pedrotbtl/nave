struct SignedDivOp {
    lhs: i8,
    rhs: i8,
    result: i8,
}

fn main(ops: [SignedDivOp; 15]) {
    for i in 0..15 {
        unsafe { verify_assert(ops[i].lhs / ops[i].rhs == ops[i].result); };
    }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
