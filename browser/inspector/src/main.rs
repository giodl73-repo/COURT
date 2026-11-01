use std::io::{self, Read};
fn main() {
    let mut text = String::new();
    if let Err(error) = io::stdin()
        .take((court_inspector::MAX_BYTES + 1) as u64)
        .read_to_string(&mut text)
    {
        eprintln!("UTF-8 input: {error}");
        std::process::exit(2);
    }
    println!("{}", court_inspector::inspect_snapshot(&text));
}
