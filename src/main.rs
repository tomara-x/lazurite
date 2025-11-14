use std::{io, thread, time::Duration};

mod eval;
use eval::*;

fn main() {
    println!("// lazurite version {} {}", env!("CARGO_PKG_VERSION"), env!("COMMIT_HASH"));
    println!("// if reading from stdin, press ctrl+d to evaluate input");
    println!("// evaluate `\"quiet\" = bool` to enable/disable printing");
    println!("// https://codeberg.org/tomara-x/lazurite");
    println!("// this machine kills fascists");
    let mut lapis = Lapis::new();
    let d = Duration::from_millis(1);
    loop {
        if let Ok(input) = io::read_to_string(io::stdin())
            && !input.is_empty()
        {
            (lapis.eval_function)(&mut lapis, &input);
        }
        thread::sleep(d);
    }
}
