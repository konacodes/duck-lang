mod lexer;
mod parser;
mod ast;
mod values;
mod interpreter;
mod builtins;
mod goose;

use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write, BufRead};

#[derive(Parser)]
#[command(name = "goose")]
#[command(about = "The Goose interpreter for Duck-lang", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a Duck file
    Run {
        /// The .duck file to run
        file: String,
    },
    /// Check a Duck file for quack issues without running
    Check {
        /// The .duck file to check
        file: String,
    },
    /// Start the interactive REPL
    Repl,
}

fn main() {
    let cli = Cli::parse();

    // Print startup message
    println!("{}", goose::startup());

    match cli.command {
        Commands::Run { file } => run_file(&file),
        Commands::Check { file } => check_file(&file),
        Commands::Repl => run_repl(),
    }
}

fn run_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => {
            println!("I can't find that file. Are you sure it exists?");
            println!("   Geese have excellent eyesight, you know.");
            return;
        }
    };

    // Lex
    let tokens = match lexer::lex(&source) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Parse
    let mut parser = parser::Parser::new(tokens);
    let blocks = match parser.parse() {
        Ok(b) => b,
        Err(errors) => {
            for e in errors {
                println!("{}", e);
            }
            return;
        }
    };

    // Execute
    let mut interpreter = interpreter::Interpreter::new();
    if let Err(e) = interpreter.run(blocks) {
        println!("{}", e);
    } else {
        println!("{}", goose::success());
    }

    // Always print rating at the end
    let (score, quip) = goose::rate_code(interpreter.stats());
    println!();
    println!("═══════════════════════════════════════");
    println!("  Goose rated your code: {}/10", score);
    println!("  \"{}\"", quip);
    println!("═══════════════════════════════════════");
}

fn check_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => {
            println!("I can't find that file. Are you sure it exists?");
            println!("   Geese have excellent eyesight, you know.");
            return;
        }
    };

    // Lex
    let tokens = match lexer::lex(&source) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Parse
    let mut parser = parser::Parser::new(tokens);
    let blocks = match parser.parse() {
        Ok(b) => b,
        Err(errors) => {
            for e in errors {
                println!("{}", e);
            }
            return;
        }
    };

    // Check for quack issues (blocks where was_quacked = false)
    let mut quack_issues = Vec::new();
    for block in &blocks {
        if !block.was_quacked {
            quack_issues.push(block.line);
        }
    }

    if quack_issues.is_empty() {
        println!("All blocks are properly quacked! Honk!");
        println!("   Your code passes the vibe check.");
    } else {
        println!("QUACK ALERT! The following lines are missing quack:");
        for line in &quack_issues {
            println!("   Line {}: No quack detected!", line);
        }
        println!();
        println!("Remember: Every block needs a quack to be valid.");
        println!("   {} issue(s) found.", quack_issues.len());
    }
}

fn run_repl() {
    println!("Welcome to the Goose REPL. Type 'exit' to leave.");
    println!("   Don't forget to quack!");
    println!();

    let stdin = io::stdin();
    let mut interpreter = interpreter::Interpreter::new();

    loop {
        print!("duck> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_err() || line.trim() == "exit" {
            println!("Goodbye! *waddles away*");
            break;
        }

        if line.trim().is_empty() {
            continue;
        }

        // Lex the line
        let tokens = match lexer::lex(line.trim()) {
            Ok(t) => t,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        // Parse the line
        let mut parser = parser::Parser::new(tokens);
        let blocks = match parser.parse() {
            Ok(b) => b,
            Err(errors) => {
                for e in errors {
                    println!("{}", e);
                }
                continue;
            }
        };

        // Execute and provide goose commentary
        for block in blocks {
            match interpreter.run_block(block) {
                Ok(result) => {
                    if let Some(value) = result {
                        println!("=> {}", value);
                    }
                    // Goose comments on the line
                    println!("   {}", goose::repl_comment());
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}
