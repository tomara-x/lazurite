use std::{io, thread, time::Duration};

mod eval;
use eval::*;

fn main() {
    println!("// lazurite version {} {}", env!("CARGO_PKG_VERSION"), env!("COMMIT_HASH"));
    println!("// https://codeberg.org/tomara-x/lazurite");
    println!("// this machine kills fascists");
    let mut lapis = Lapis::new();
    loop {
        if let Ok(input) = io::read_to_string(io::stdin()) {
            lapis.eval(&input);
        }
        thread::sleep(Duration::from_nanos(1));
    }
}
