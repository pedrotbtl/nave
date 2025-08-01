fn main(x: u128, y: u128, z: u8) {
    let const_x = 12345;
    let const_y = 2345678;
    let const_z = 2;

    assert(x + y == const_x + const_y);
    assert(y - x == const_y - const_x);
    assert(x * y == const_x * const_y);
    assert(y / x == const_y / const_x);
    assert(y % x == const_y % const_x);
    assert(!x == !const_x);
    assert(x ^ y == const_x ^ const_y);
    assert(x & y == const_x & const_y);
    assert(x | y == const_x | const_y);
    assert(x >> z == const_x >> const_z);
    assert(x << z == const_x << const_z);
    assert(x < y == const_x < const_y);
    assert(x <= y == const_x <= const_y);
    assert(x != y == const_x != const_y);
    assert(y > x == const_y > const_x);
    assert(y >= x == const_y >= const_x);
}
unconstrained fn verify_assert(b: bool) { assert(false); }
