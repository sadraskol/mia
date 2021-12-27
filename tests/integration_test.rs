use std::process::Command;

#[test]
fn first_test() {
    let out = Command::new("target/debug/mia")
        .arg("tests/samples/00001.m")
        .output()
        .unwrap();
    assert_eq!(String::from_utf8(out.stdout).unwrap(), "{\"name\":\"Some name\",\"comment\":\"12312\",\"tags\":[\"tag 1\",\"tag 2\"],\"opt\":41.82}\n");
}

#[test]
fn second_test() {
    let out = Command::new("target/debug/mia")
        .arg("tests/samples/00002.m")
        .output()
        .unwrap();
    assert_eq!(String::from_utf8(out.stdout).unwrap(), "");
    assert_eq!(String::from_utf8(out.stderr).unwrap(), "Literal array can only have a single type\n");
}

#[test]
fn third_test() {
    let out = Command::new("target/debug/mia")
        .arg("tests/samples/00003.m")
        .output()
        .unwrap();
    assert_eq!(String::from_utf8(out.stdout).unwrap(), "[\
    {\"home\":\"/home/bob\",\"public_key\":\"/home/bob/.ssh/id_ed25519.pub\",\"private_key\":\"/home/bob/.ssh/id_ed25519\"},\
    {\"home\":\"/home/alice\",\"public_key\":\"/home/alice/.ssh/id_ed25519.pub\",\"private_key\":\"/home/alice/.ssh/id_ed25519\"}]\n");
}
