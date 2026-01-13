// Tests for SAT (falsified) cases

#[macro_use]
mod helper;

use acir_checker::BackendType;
use crate::helper::is_falsified;

#[test]
fn test_assert_false() {
    let source = add_decl!(
        "fn main() {
            unsafe { verify_assert(false); }
        }");
    assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_mul() {
    #[allow(unused)]
    let source = add_decl!(
        "fn main(x: Field, y: Field) {
            let z = x * y;
            unsafe { verify_assert(z != x * y); }
        }");

    assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_add() {
    #[allow(unused)]
    let source = add_decl!(
        "fn main(x: Field, y: Field) {
            let z = x + y;
            unsafe { verify_assert(z != x + y); }
        }");

    assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_div() {
    #[allow(unused)]
    let source = add_decl!(
        "fn main(x: Field, y: Field) {
            let z = x / y;
            unsafe { verify_assert(z != x / y); }
        }");

    assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_sub() {
    #[allow(unused)]
    let source = add_decl!(
        "fn main(x: Field, y: Field) {
            let z = x - y;
            unsafe { verify_assert(z != x - y); }
        }");

    assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_sub_u8() {
    #[allow(unused)]
    let source = add_decl!(
        "fn main(x: u8, y: u8) {
            let z = x - y;
            unsafe { verify_assert(z != x - y); }
        }");

    // Timeout with ff-gb backend
    // assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_add_u8() {
    #[allow(unused)]
    let source = add_decl!(
        "fn main(x: u8, y: u8) {
            let z = x + y;
            unsafe { verify_assert(z != x + y); }
        }");

    // Timeout with ff-gb backend
    // assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_mul_u8() {
    #[allow(unused)]
    let source = add_decl!(
        "fn main(x: u8, y: u8) {
            let z = x * y;
            unsafe { verify_assert(z != x * y); }
        }");

    // Timeout with ff-gb backend
    // assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_div_u8() {
    #[allow(unused)]
    let source = add_decl!(
        "fn main(x: u8, y: u8) {
            let z = x / y;
            unsafe { verify_assert(z != x / y); }
        }");

    // Timeout to solve with ff-gb and ff-split backends
    // assert!(is_falsified(source, BackendType::FfGb, false));
    // assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_and_u8() {
    #[allow(unused)]
    let source = add_decl!(
        "fn main(x: u8, y: u8) {
            let z = x & y;
            unsafe { verify_assert(z != x & y); }
        }");

    assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_or_u8() {
    #[allow(unused)]
    let source = add_decl!(
        "fn main(x: u8, y: u8) {
            let z = x | y;
            unsafe { verify_assert(z != x | y); }
        }");

    assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_xor_u8() {
    #[allow(unused)]
    let source = add_decl!(
        "fn main(x: u8, y: u8) {
            let z = x ^ y;
            unsafe { verify_assert(z != x ^ y); }
        }");

    assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_xor_zero_u8() {
    #[allow(unused)]
    let source = add_decl!(
        "fn main(x: u8) {
            let z = x ^ x;
            unsafe { verify_assert(z != 0); }
        }");

    assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_xor_zero_assert_u8() {
    #[allow(unused)]
    let source = add_decl!(
        "fn main(x: u8, y: u8) {
            assert(x == y);
            let z = x ^ y;
            let w = z & x;
            let v = z & y;
            unsafe { verify_assert((w != 0) & (v != 0)); }
        }");

    // Timeout with ff-gb backend
    // assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_comparison_u8() {
    let source = add_decl!(
        "fn main(x: u8, y: u8) {
            let z = x > y;
            unsafe { verify_assert(z != (x > y)); }
        }");

    assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_not_bool() {
    let source = add_decl!(
        "fn main(x: bool) {
            let z = !x;
            unsafe { verify_assert(z == x); }
        }");

    assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_array_add() {
    let source = add_decl!(
        "fn main(x: [u8; 2]) {
            let z = x[0] + x[1];
            unsafe { verify_assert(z != x[0] + x[1]); }
        }");
    
    // Timeout with ff-gb
    // assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}

#[test]
fn test_loop_add() {
    let source = add_decl!(
        "fn main(values: [Field; 2]) {
            let mut sum = 0;
            for value in values {
                sum = sum + value;
            }
            unsafe { verify_assert(sum != (values[0] + values[1])); }
        }");

    assert!(is_falsified(source, BackendType::FfGb, false));
    assert!(is_falsified(source, BackendType::FfSplit, false));
    assert!(is_falsified(source, BackendType::Int, false));
}