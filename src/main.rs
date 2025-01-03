use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        match command {
            "exit" => {
                println!("Goobmight!");
                break;
            }
            "echo" => {
                println!("{}", args.join(" "));
            }
            "" => continue,
            _ => println!("Unknown command: {}", command),
        }
    }
}