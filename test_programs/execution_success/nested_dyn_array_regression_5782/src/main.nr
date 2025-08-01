fn main(mut array: [Field; 2], i: u32) {
    assert(array[i - 1] == 5);
    assert(array[i] == 10);

    array[i] = 2;

    let array2 = [array, array];

    assert(array2[0][0] == 5);
    assert(array2[0][i] == 2);
    assert(array2[i][0] == 5);
    unsafe { verify_assert(array2[i][i] == 2); };
}
unconstrained fn verify_assert(b: bool) { assert(false); }
