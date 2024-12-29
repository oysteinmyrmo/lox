fn usage() {
    println!("Usage: lox <path to script>");
}

fn run(source: &String) {
    let lines = source.lines();
    for line in lines {
        println!("{}", line);
    }
}

fn run_file(path: &String) {
    match std::fs::read_to_string(path) {
        Ok(source) => {
            run(&source);
        }
        Err(e) => {
            println!("Error reading file {:?}: {}", path, e);
            std::process::exit(-1);
        }
    }
}

fn run_prompt() {
    let mut buffer = String::new();
    buffer.reserve(128); // Assume max 128 characters per line initially.

    loop {
        buffer.clear();
        print!("> ");
        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                run(&buffer);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}

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
