use learn_rust::add_two;

#[test] // <- #[test] attr is automatically added by `rust-analyzer` (only when auto-format is enabled?)
fn it_should_add_two() {
    let result = add_two(3);
    assert_eq!(result, 5);
}
