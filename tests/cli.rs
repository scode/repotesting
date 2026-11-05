use std::process::Command;

/// The version output is a machine-readable contract, including its newline.
#[test]
fn sole_version_argument_prints_package_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_repotesting"))
        .arg("--version")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        output.stdout,
        format!("repotesting {}\n", env!("CARGO_PKG_VERSION")).as_bytes()
    );
    assert!(output.stderr.is_empty());
}

/// Other argument lists must keep the greeting, even when they contain the flag.
#[test]
fn other_arguments_keep_the_greeting() {
    for args in [
        &[][..],
        &["other"][..],
        &["--version", "other"][..],
        &["other", "--version"][..],
        &["--version", "--version"][..],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_repotesting"))
            .args(args)
            .output()
            .unwrap();

        assert_eq!(output.status.code(), Some(0), "args: {args:?}");
        assert_eq!(output.stdout, b"Hello, world!\n", "args: {args:?}");
        assert!(output.stderr.is_empty(), "args: {args:?}");
    }
}
