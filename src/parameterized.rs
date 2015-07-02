//! Parameterized test utilities.

/// Generates test functions for a set of parameters.
///
/// This functionality is useful when testing requires running
/// the same block of test code, but need to tweak a few
/// input parameters to it.
///
/// This macro generates a new test function for each set of
/// parameters, so that they show up as separate tests when
/// running `cargo test`. This allows the developer to tell
/// exactly which set of parameters cause a test to fail.
///
/// # Examples
///
/// ```no_run
/// # #[macro_use] extern crate corrosion;
/// fn test_add(a : i32, b : i32, expected_result : i32) {
///   assert_eq!(expected_result, a + b);
/// }
///
/// ptest!(test_add [
///     add_zero (0, 5, 5),
///     add_positive_and_negative (-8, 3, -5),
///     add_big_numbers (51432, 765437, 816869)
/// ]);
///
/// # fn main() {
/// # }
///
/// ```
#[macro_export]
macro_rules! ptest {
    ($fn_name : ident
        [$($test_name : ident ($($test_param : expr),*)),*])
    => (
        $(
            #[test]
            fn $test_name() {
                $fn_name($($test_param,)*)
            }
        )*
    )
}