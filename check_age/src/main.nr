fn main(age: u8) -> pub bool {
    assert(age != 0);
    let mut result = false;

    if age > 17 {
        result = true;
    }
    result
}
