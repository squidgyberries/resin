fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let target = args.get_mut(2).unwrap();
    if !target.ends_with("/") && !target.ends_with("\\") {
        target.push_str("/");
    }
    resin_core::extract(&args[1], &args[2]);
}
