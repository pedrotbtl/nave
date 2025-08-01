// Regression for issue #7710 (https://github.com/noir-lang/noir/issues/7710)
unconstrained fn foo() -> [Field; 0] {
    println("foo");
    []
}

fn main() {
    // Safety: testing context
    let x = unsafe { foo() };

    let _ = x.as_slice().map(|_| {
        assert(false);
        panic(f"test");

        assert(false);
        assert_constant(x);
        0
    });

    let _ = x.as_slice().mapi(|_, _| {
        assert(false);
        panic(f"test");

        assert(false);
        assert_constant(x);
        0
    });

    x.as_slice().for_each(|_| {
        assert(false);
        panic(f"test");

        assert(false);
        assert_constant(x);
    });

    x.as_slice().for_eachi(|_, _| {
        assert(false);
        panic(f"test");

        assert(false);
        assert_constant(x);
    });

    for _ in x.as_slice() {
        assert(false);
        panic(f"test");

        assert(false);
        std::assert_constant(x);
    }

    let _ = x.map(|_| {
        assert(false);
        assert(false);
        panic(f"test");
        assert_constant(x);
    });

    for _ in x {
        assert(false);
        assert(false);
        panic(f"test");
        assert_constant(x);
    }

    let empty_array: [Field; 0] = [];
    for _ in empty_array.as_slice() {
        assert(false);
        assert(false);
        panic(f"test");
        assert_constant(x);
    }

    for _ in 0..0 {
        assert(false);
        unsafe { verify_assert(false); }
        panic(f"test");
        assert_constant(x);
    }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
