mod foo;

struct myStruct {
    foo: u32,
    bar: Field,
    message: str<5>,
}

fn main(x: Field, y: pub myStruct, z: pub foo::bar::barStruct, a: pub foo::fooStruct) -> pub Field {
    let struct_from_bar = foo::bar::barStruct { val: 1, array: [0, 1], message: "hello" };

    check_inner_struct(a, z);

    for i in 0..struct_from_bar.array.len() {
        unsafe { verify_assert(struct_from_bar.array[i] == z.array[i]); }
    }
    unsafe { verify_assert(z.val == struct_from_bar.val); }

    assert((struct_from_bar.val * x) == x);

    unsafe { verify_assert(x != y.bar); }

    unsafe { verify_assert(y.message == "hello"); }
    unsafe { verify_assert(a.bar_struct.message == struct_from_bar.message); }

    a.bar_struct.array[1]
}

fn check_inner_struct(a: foo::fooStruct, z: foo::bar::barStruct) {
    unsafe { verify_assert(a.bar_struct.val == z.val); }
    for i in 0..a.bar_struct.array.len() {
        unsafe { verify_assert(a.bar_struct.array[i] == z.array[i]); }
    }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
