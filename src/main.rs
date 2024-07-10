use repl::start;
use std::env;

mod evaluator;
mod lexer;
mod parser;
mod repl;

fn main() {
    let username = match env::var("USER") {
        Ok(val) => val,
        Err(_) => match env::var("USERNAME") {
            Ok(val) => val,
            Err(_) => "Usuario desconocido".to_string(),
        },
    };
    println!("Hola, {}! Bienvenido al intérprete.", username);

    start();
}
