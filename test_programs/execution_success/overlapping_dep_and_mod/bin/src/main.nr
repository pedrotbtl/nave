// bin/main.nr

fn main() -> pub Field {
    assert(foo::bar() + dep::foo::bar() == 11);
    assert(foo::bar() == 5);
    unsafe { verify_assert(dep::foo::bar() == 6); }
    foo::bar() + dep::foo::bar()
}

mod foo {
    pub(crate) fn bar() -> Field {
        5
    }
}
unconstrained fn verify_assert(b: bool) {}
