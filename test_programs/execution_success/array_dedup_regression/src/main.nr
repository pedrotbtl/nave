unconstrained fn main(x: u32) {
    let a1 = [1, 2, 3, 4, 5];

    for i in 0..5 {
        let mut a2 = [1, 2, 3, 4, 5];
        a2[x + i] = 128;
        println(a2);

        if i != 0 {
            assert(a2[x + i - 1] != 128);
        }
    }

    // Can't use `== [1, 2, 3, 4, 5]` here, that make_array may get
    // deduplicated to equal a1 in the bugged version
    assert(a1[0] == 1);;
    assert(a1[1] == 2);;
    assert(a1[2] == 3);;
    assert(a1[3] == 4);;
    unsafe { verify_assert(a1[4] == 5); };
}
unconstrained fn verify_assert(b: bool) { assert(false); }
