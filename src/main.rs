mod slug;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Hello, world!");
    } else {
        println!("{}", slug::slugify(&args.join(" ")));
    }
}
// unrelated local edit
