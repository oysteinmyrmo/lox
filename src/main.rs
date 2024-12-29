mod scanner;

use scanner::scanner::Scanner;

fn usage() {
    println!("Usage: lox <path to script>");
}

fn report(line: u32, location: String, message: String) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
}

unsafe fn error(line: u32, message: String) {
    report(line, "".into(), message);
    HAD_ERROR = true;
}

unsafe fn run(source: &String) {
    let lines = source.lines();
    for line in lines {
        println!("{}", line);
    }

    let scanner: Scanner = Scanner::new(source.clone());

    if HAD_ERROR {
        std::process::exit(65);
    }
}

fn run_file(path: &String) {
    match std::fs::read_to_string(path) {
        Ok(source) => unsafe {
            run(&source);
        },
        Err(e) => {
            eprintln!("Error reading file {:?}: {}", path, e);
            std::process::exit(-1);
        }
    }
}

fn run_prompt() {
    let mut line = String::new();
    line.reserve(128); // Assume max 128 characters per line initially.

    loop {
        line.clear();
        print!("> ");
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => unsafe {
                run(&line);
                HAD_ERROR = false;
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

// Note: This should be moved into the actual lexer later on. Following the book for now.
// Note: When moving this, all the related unsafes can be removed too.
static mut HAD_ERROR: bool = false;

fn main() {
    // Skip first argument since it is the path to the executable itself.
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() > 1 {
        usage();
        std::process::exit(64); // Taken from sysexits.h
    } else if args.len() == 1 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}
