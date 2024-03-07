extern crate cargo_test_targets;

#[test]
fn integration() {
    assert_eq!(cargo_test_targets::add(2, 2), 5);
}
