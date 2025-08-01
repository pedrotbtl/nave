global N: Field = 2;

pub(crate) struct barStruct {
    pub(crate) val: Field,
    pub(crate) array: [Field; 2],
    pub(crate) message: str<5>,
}
unconstrained fn verify_assert(b: bool) {}
