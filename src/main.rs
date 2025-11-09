use std::{io, thread, time::Duration};

mod eval;
use eval::*;

fn main() {
    println!("// lazurite version {} {}", env!("CARGO_PKG_VERSION"), env!("COMMIT_HASH"));
    println!("// https://codeberg.org/tomara-x/lazurite");
    println!("// this machine kills fascists");
    let mut lapis = Lapis::new();
    let mut input = String::new();
    loop {
        if io::stdin().read_line(&mut input).is_ok() {
            lapis.eval(&input);
            input.clear();
        }
        thread::sleep(Duration::from_nanos(1));
    }
}
