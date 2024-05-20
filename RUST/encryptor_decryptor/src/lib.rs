use std::{error::Error, io, process};

#[derive(Debug)]
enum Process {
    Encryption,
    Decryption,
    Help,
}

#[derive(Debug)]
pub struct Config {
    filename: Option<String>,
    operation: Process,
    password: Option<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let mut config = parse_args(&args).unwrap_or_else(|err| {
            eprintln!("Error parsing arguments: {}", err);
            print_help();
            process::exit(1);
        });

        if config.password.is_none() {
            let mut password = String::new();

            println!("Please enter the password: ");
            io::stdin()
                .read_line(&mut password)
                .expect("failed to read line");

            config.password = Some(password.trim().to_owned());
        }

        Ok(config)
    }
}

fn print_help() {
    println!(
        "
Usage: cargo run [OPTION]
    -h, --help      : print help message
    -e, --encrypt   : encrypt the file
    -d, --decrypt   : decrypt the file
    -p, --password  : set the password
    -f, --file      : set the file to encrypt/decrypt
    "
    );
}

fn parse_args(args: &[String]) -> Result<Config, &'static str> {
    let mut config = Config {
        operation: Process::Help,
        password: None,
        filename: None,
    };

    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => config.operation = Process::Help,
            "-e" | "--encrypt" => config.operation = Process::Encryption,
            "-d" | "--decrypt" => config.operation = Process::Decryption,
            "-p" | "--password" => {
                if i + 1 < args.len() {
                    config.password = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    return Err("Missing value for file");
                }
            }
            "-f" | "--file" => {
                if i + 1 < args.len() {
                    config.filename = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    return Err("Missing value for file");
                }
            }
            _ => return Err("Unknown argument"),
        }

        i += 1;
    }

    Ok(config)
}

fn encryption(config: &Config) {}

fn decryption(config: &Config) {}

pub fn run(config: &Config) {
    match config.operation {
        Process::Encryption => encryption(config),
        Process::Decryption => decryption(config),
        Process::Help => print_help(),
    }
}
