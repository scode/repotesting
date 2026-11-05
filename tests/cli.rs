use std::process::Command;

/// The binary retains its greeting when invoked without arguments.
#[test]
fn no_arguments_prints_greeting() {
    let output = Command::new(env!("CARGO_BIN_EXE_repotesting"))
        .output()
        .expect("run binary");

    assert!(output.status.success());
    assert_eq!(output.stdout, b"Hello, world!\n");
}

/// Multiple arguments form one input to the slugifier, including punctuation.
#[test]
fn arguments_print_slug() {
    let output = Command::new(env!("CARGO_BIN_EXE_repotesting"))
        .args(["Hello", "World!!"])
        .output()
        .expect("run binary");

    assert!(output.status.success());
    assert_eq!(output.stdout, b"hello-world\n");
}
