/// Print the package version only for a sole `--version` argument.
/// Every other argument list retains the original greeting.
fn main() {
    let mut args = std::env::args_os().skip(1);
    if args.next().as_deref() == Some(std::ffi::OsStr::new("--version")) && args.next().is_none() {
        println!("repotesting {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    println!("Hello, world!");
}
// unrelated local edit
