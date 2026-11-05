use std::process::Command;

/// The verbose response is reserved for the flag as the only argument;
/// all other invocations retain the original output and successful exit.
#[test]
fn argument_contract() {
    for (args, expected) in [
        (vec!["--verbose"], b"verbose mode on\n".as_slice()),
        (vec![], b"Hello, world!\n".as_slice()),
        (vec!["--unknown"], b"Hello, world!\n".as_slice()),
        (vec!["--verbose", "extra"], b"Hello, world!\n".as_slice()),
        (
            vec!["--verbose", "--verbose"],
            b"Hello, world!\n".as_slice(),
        ),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_repotesting"))
            .args(&args)
            .output()
            .expect("run built binary");
        assert_eq!(output.status.code(), Some(0), "args: {args:?}");
        assert_eq!(output.stdout, expected, "args: {args:?}");
        assert!(output.stderr.is_empty(), "args: {args:?}");
    }
}
