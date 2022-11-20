mod executor;
mod lexer;
mod parser;

use std::collections::HashMap;

pub fn run_file(source: &str) {
    let mut tokens = lexer::Lexer::new(source, HashMap::new(), 0);

    match parser::parse(source, &mut tokens) {
        Ok(parsed) => {
            let mut exec = executor::Executor::new(tokens.literal_table, tokens.symbol_lookup);
            if let Err(message) = exec.execute(&parsed) {
                println!("\n**[Execution Failed]**");
                if let Some(region) = source.get((message.0).0..=(message.0).1) {
                    println!("{}", region);
                }
                println!("^^^^{}", message.1);
            }
        }
        Err(message) => {
            println!("{}", message);
            std::process::exit(1);
        }
    }
}
