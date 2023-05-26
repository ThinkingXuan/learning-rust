use test_test;

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, test_test::add(1, 3));
}