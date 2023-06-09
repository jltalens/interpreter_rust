use std::io::{Write, BufRead};

use crate::lexer::lexer::Lexer;

pub fn repl(stdin: impl BufRead, mut stdout: impl Write) {
    write!(stdout, ">> ").expect("Failed to write to stdout");
    stdout.flush().expect("Failed to flush stdout");
    for line in stdin.lines() {
        let lexer = Lexer::new(line.unwrap());
        for token in lexer {
            writeln!(stdout, "{:?}", token).expect("Failed to write to stdout");
        }
        write!(stdout, ">> ").expect("Failed to write to stdout");
        stdout.flush().expect("Failed to flush stdout");
    }
}

