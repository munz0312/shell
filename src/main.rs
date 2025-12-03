use std::{
    env,
    io::{self, Write},
    process::Command,
};

fn main() {
    loop {
        let inp = read_input();
        let tokens = parse_input(inp);
        exec(tokens);
    }
}

fn read_input() -> String {
    let cwd = std::env::current_dir().unwrap();
    print!("{} $ ", cwd.display());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");
    input
}

fn parse_input(raw: String) -> Vec<String> {
    raw.split_ascii_whitespace()
        .map(|piece| piece.to_string())
        .collect()
}

fn exec(tokens: Vec<String>) {
    let len = tokens.len();
    if len < 1 {
        return;
    };
    let cmd = &tokens[0];
    match cmd.as_str() {
        "cd" => {
            if len < 2 {
                cd(std::env::var("HOME").unwrap().as_str())
            } else if len > 2 {
                eprintln!("cd: Too many arguments")
            } else {
                cd(&tokens[1])
            }
        }
        "exit" => std::process::exit(0),
        _ => {
            let _ = Command::new(cmd)
                .args(&tokens[1..])
                .status()
                .expect("Failed to execute command");
        }
    }
}

fn cd(path: &str) {
    if let Err(_) = env::set_current_dir(path) {
        eprintln!("No such directory: {}", path);
    }
}
