use calculate_macro::calc;

#[test]
fn test_cal_addition() {
    let result = calc!(u8::MAX + u16::MAX; u32);
    assert_eq!(result, 65790u32);
}

#[test]
fn test_cal_subtraction() {
    let result = calc!(30u8 - 10u8; u16);
    assert_eq!(result, 20u16);
}

#[test]
fn test_cal_multiplication() {
    let result = calc!(5u8 * 6u8; u32);
    assert_eq!(result, 30u32);
}

#[test]
fn test_cal_division() {
    let result = calc!(20u8 / 4u8; u16);
    assert_eq!(result, 5u16);
}

#[test]
fn test_cal_remainder() {
    let result = calc!(20u8 % 7u8; u16);
    assert_eq!(result, 6u16);
}

#[test]
fn test_cal_variable() {
    let a = 255u8;
    let result = calc!(a + 10u8; u16);
    assert_eq!(result, 265u16);
}

#[test]
fn test_cal_nested_expression() {
    let result = calc!((10u8 + 20u8) * 2; u32);
    assert_eq!(result, 60u32);
}

#[test]
fn test_cal_function_call() {
    #[allow(unused_imports)]
    use core::cmp::max;
    let result = calc!(max(10u8, 20u8); u16);
    assert_eq!(result, 20u16);
}

#[test]
fn test_cal_negative_value() {
    let result = calc!(-10u8; i16);
    assert_eq!(result, -10i16);
}
