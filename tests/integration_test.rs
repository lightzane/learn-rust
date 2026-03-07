use learn_rust::add_two;

mod common;
mod utils;

#[test] // <- #[test] attr is automatically added by `rust-analyzer` (only when auto-format is enabled?)
fn it_adds_two() {
    common::setup();
    utils::some_util();

    let result = add_two(3);
    assert_eq!(result, 5);
}
