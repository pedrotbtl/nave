pub trait Trait {
    let N: u32;

    fn foo() -> u32;
}

struct Foo<let A: u32, let B: u32> {}

impl<let A: u32, let B: u32> Trait for Foo<A, B> {
    let N: u32 = A + B;

    fn foo() -> u32 {
        Self::N
    }
}

fn main() {
    let x = Foo::<10, 20>::foo();
    unsafe { verify_assert(x == 30); };

    comptime {
        let x = Foo::<10, 20>::foo();
        unsafe { verify_assert(x == 30); };
    }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
