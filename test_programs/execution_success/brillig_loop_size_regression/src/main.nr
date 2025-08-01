struct EnumEmulation {
    a: Option<Field>,
    b: Option<Field>,
    c: Option<Field>,
}

unconstrained fn main() -> pub Field {
    let mut emulated_enum =
        EnumEmulation { a: Option::some(1), b: Option::none(), c: Option::none() };

    for _ in 0..1 {
        unsafe { verify_assert(emulated_enum.a.unwrap() == 1); };
    }

    emulated_enum.a = Option::some(2);
    emulated_enum.a.unwrap()
}
unconstrained fn verify_assert(b: bool) { assert(false); }
