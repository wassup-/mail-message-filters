mod configuration;
mod dat;
mod printer;
mod util;
mod xml;

#[derive(clap::Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Lint,
    Print { format: PrintFormat },
}

#[derive(clap::ValueEnum, Copy, Clone, Debug)]
enum PrintFormat {
    Evolution,
    Thunderbird,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    return match cli.command {
        Command::Lint => lint_config(),
        Command::Print { format } => print_config(format),
    };
}

fn load_config() -> Result<Configuration, Box<dyn std::error::Error>> {
    use std::io::{self, Read};

    let mut buff = Vec::new();
    let _ = io::stdin().read_to_end(&mut buff)?;

    let config = serde_yaml::from_slice(&buff)?;
    Ok(config)
}

fn lint_config() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    dbg!(config);

    Ok(())
}

fn print_config(format: PrintFormat) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    let output = match format {
        PrintFormat::Evolution => printer::evolution::print_config(config),
        PrintFormat::Thunderbird => printer::thunderbird::print_config(config),
    };

    println!("{}", output);

    Ok(())
}

use std::io::BufReader;

use clap::Parser;
use configuration::Configuration;
