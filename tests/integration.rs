use std::process::Command;
use std::str;

const EXE: &str = env!("CARGO_BIN_EXE_foo");

#[test]
fn test_foo() {
    let cmd = Command::new(EXE).output().unwrap().stdout;
    let output = str::from_utf8(&cmd).unwrap();
    assert_eq!(output.lines().next().unwrap().trim(), "HTTP/1.0 200 OK");
}
