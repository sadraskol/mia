use std::process::Command;

#[test]
fn first_test() {
    let out = Command::new("target/debug/mia")
        .arg("tests/samples/00001.m")
        .output()
        .unwrap();
    let result = String::from_utf8(out.stdout).unwrap();
    assert_eq!(result, "{\"name\":\"Some name\",\"comment\":\"12312\",\"tags\":[\"tag 1\",\"tag 2\"],\"opt\":41.82}\n")
}