pub fn from_baz(x: [Field; crate::foo::MAGIC_NUMBER]) {
    for i in 0..crate::foo::MAGIC_NUMBER {
        unsafe { verify_assert(x[i] == crate::foo::MAGIC_NUMBER as Field); }
    }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
