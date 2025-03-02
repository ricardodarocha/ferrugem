mod environment;
mod expr;
mod interpreter;
mod parser;
mod resolver;
mod scanner;
mod stmt;
mod tests;
use crate::interpreter::*;
use crate::parser::*;
use crate::resolver::*;
use crate::scanner::*;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process::exit;

pub fn run_file(path: &str) -> Result<(), String> {
    // let mut interpreter = Interpreter::new();
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run_string(&contents),
    }
}

pub fn run_string(contents: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();

    run(&mut interpreter, contents)
}

#[test]
fn test_run_string() {
    for exp in ["saida \"ola\";",] {
        match run_string(exp) {
                Ok(_) => exit(0),
                Err(msg) => {
                    println!("ERRO:\n{msg}");
                    exit(1);
                }
            }
    }
}

fn run(interpreter: &mut Interpreter, contents: &str) -> Result<(), String> {

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);//volta o cursor;
    print!("[Fe] Ferrugem vs 0.1 🟠 \nPortugol sendo reescrito em Rust\n==================================\n");

    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;

    let resolver = Resolver::new();
    let locals = resolver.resolve(&stmts.iter().collect())?;

    interpreter.resolve(locals);

    interpreter.interpret(stmts.iter().collect())?;
    interpreter.doc();
    return Ok(());
}

fn run_prompt() -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    loop {
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Não foi possível limpar a saída".to_string()),
        }

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("");
                    return Ok(());
                } else if n == 1 {
                    continue;
                }
            }
            Err(_) => return Err("Não foi possível capturar a entrada".to_string()),
        }

        println!("ECO: {}", buffer);
        match run(&mut interpreter, &buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("🔴[Fe] ERRO:\n{}", msg);
                exit(1);
            }
        }
    } else if args.len() == 3 && args[1] == "e" {
        match run_string(&args[2]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("🔴[Fe] ERRO:\n{msg}");
                exit(1);
            }
        }
    } else if args.len() == 1 {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("🔴[Fe] ERRO\n{}", msg);
                exit(1);
            }
        }
    } else {
        println!("🔴[Fe] Ferrugem Falhou");
        exit(64);
    }
}
