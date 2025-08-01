use crate1::MyTrait;

fn main(x: Field, y: pub Field) {
    let mut V = crate2::MyStruct { Q: x };
    V.Add10();
    unsafe { verify_assert(V.Q == y); }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
