use std::io;

use interpreter::repl::repl::repl;

fn main() {
    println!("Hello, This is the Monkey programming language!");
    repl(io::stdin().lock(), io::stdout());
}
