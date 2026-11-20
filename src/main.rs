fn main() {
    // Match OS strings so non-Unicode arguments keep the default behavior.
    let mut args = std::env::args_os().skip(1);
    if args.next().as_deref() == Some(std::ffi::OsStr::new("--verbose")) && args.next().is_none() {
        println!("verbose mode on");
    } else {
        println!("Hello, world!");
    }
}
// unrelated local edit
