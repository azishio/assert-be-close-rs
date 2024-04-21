use std::fmt::Display;

use num::Float;

/// Asserts that two decimals are equal with the specified precision.
/// On panic, the function will print the value of the decimal with their debug representations.
/// 2つの小数が指定した精度で等しいことを保証する。
/// パニックになると、この関数は小数の値をデバッグ表現とともに出力する。
///
/// # Panics
///
/// Panic if the value is not equivalent to the specified precision.
/// 値が指定した精度で等価でなければパニックする。
///
/// # Examples
///
/// ```should_panic
/// use assert_be_close::assert_be_close;
///
/// assert_be_close(1.0, 1.0001, 3);
/// assert_be_close(1.0, 1.0001, 4); // panic
/// ```
pub fn assert_be_close<T: Float + Display>(left: T, right: T, precision: i32) {
    let expected_diff = T::from(10.0).unwrap().powi(-precision) / T::from(2.0).unwrap();
    let received_diff = (left - right).abs();

    if received_diff >= expected_diff {
        panic!("assertion 'left ≈ right` failed\n left: {}\nright: {}\nreceived_diff: {}\nexpected_diff: {}", left, right, received_diff, expected_diff);
    }
}

/// Asserts that two decimals aren't equal with the specified precision.
/// On panic, the function will print the value of the decimal with their debug representations.
/// 2つの小数が指定した精度で等しくないことを保証する。
/// パニックになると、この関数は小数の値をデバッグ表現とともに出力する。
///
/// # Panics
///
/// Panic if the value is equivalent to the specified precision.
/// 値が指定した精度で等価であるとパニックする。
///
/// # Examples
///
/// ```should_panic
/// use assert_be_close::assert_not_close;
///
/// assert_not_close(1.0, 1.0001, 4);
/// assert_not_close(1.0, 1.0001, 3); // panic
/// ```
pub fn assert_not_close<T: Float + Display>(left: T, right: T, precision: i32) {
    let expected_diff = T::from(10.0).unwrap().powi(-precision) / T::from(2.0).unwrap();
    let received_diff = (left - right).abs();

    if received_diff <= expected_diff {
        panic!("assertion 'left != right` failed\n left: {}\nright: {}\nreceived_diff: {}\nexpected_diff: {}", left, right, received_diff, expected_diff);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn assert_be_close_works() {
        assert_be_close(1.0, 1.0001, 3);
    }

    #[test]
    fn assert_not_close_works() {
        assert_not_close(1.0, 1.0001, 4);
    }
}
