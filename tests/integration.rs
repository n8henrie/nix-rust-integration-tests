use std::process::Command;
use std::str;

const EXE: &str = env!("CARGO_BIN_EXE_foo");

#[test]
fn test_foo() {
    let status = Command::new(EXE).status();
    assert!(status.is_ok());
}
