mod expressions;
mod moon;
mod tokentype;
mod token;
mod scanner;
mod value;
mod moonenv;
mod statements;
mod interpreter;

use std::env;

use crate::moon::Moon;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut moon: Moon = Moon::new();

    if args.len() > 2 {
        println!("Pouziti: moon [skript].");
        std::process::exit(64);
    } else if args.len() == 2 {
        let source: &String = &args[1];
        moon.run_file(source);
    } else {
        moon.run_prompt();
    }
}
