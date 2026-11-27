use std::process::Command;

#[test]
fn version_prints_release_and_schema() {
    let output = Command::new(env!("CARGO_BIN_EXE_ca-server"))
        .arg("--version")
        .output()
        .expect("failed to run ca-server");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("ca-server {} (API schema 1)\n", env!("CARGO_PKG_VERSION"))
    );
}
