pub(crate) mod bar;

pub(crate) struct fooStruct {
    pub(crate) bar_struct: bar::barStruct,
    pub(crate) baz: Field,
}
unconstrained fn verify_assert(b: bool) { assert(false); }
