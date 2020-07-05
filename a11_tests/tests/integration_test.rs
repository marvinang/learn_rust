use a11_tests; 

mod common;

#[test]
fn tt() {
    common::setup();
    common::user::hah();
    assert_eq!(4, a11_tests::add_two(2));
}
