// Have to use inputs otherwise the ACIR gets optimized away due to constants.
// With the original ACIR the optimizations can rearrange or merge opcodes,
// which might end up getting out of sync with assertion messages.
// #[test(should_fail_with = "first")]
fn main(a: Field, b: Field, c: Field) {
    if (a + b) * (a - b) != c {
        assert((a + b) * (a - b) == c, "first");
        assert((a - b) * (a + b) == c, "second");
        assert((2 * a + b) * (a - b / 2) == c * c, "fourth");
        unsafe { verify_assert((a + b) * (a - b) == c);}
        // unsafe { verify_assert((2 * a + b) * (a - b / 2) == c * c); }
    } else {
        // The fuzzer might have generated a passing test.
        assert((a + b) * (a - b) != c, "first");
        // unsafe { verify_assert((a + b) * (a - b) != c); }
    }
}

unconstrained fn verify_assert(b: bool) { assert(false); }
