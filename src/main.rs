mod configuration;
mod dat;
mod printer;
mod xml;

#[derive(clap::Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Lint { path: String },
    Print { path: String, format: PrintFormat },
}

#[derive(clap::ValueEnum, Copy, Clone, Debug)]
enum PrintFormat {
    Evolution,
    Thunderbird,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    return match cli.command {
        Command::Lint { path } => lint_config(path),
        Command::Print { path, format } => print_config(path, format),
    };
}

fn load_config(path: String) -> Result<Configuration, Box<dyn std::error::Error>> {
    use std::{fs::File, io::BufReader};

    let f = File::open(path)?;
    let rdr = BufReader::new(f);
    let config = serde_yaml::from_reader(rdr)?;
    Ok(config)
}

fn lint_config(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config(path)?;

    dbg!(config);

    Ok(())
}

fn print_config(path: String, format: PrintFormat) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config(path)?;

    let output = match format {
        PrintFormat::Evolution => printer::evolution::print_config(config),
        PrintFormat::Thunderbird => printer::thunderbird::print_config(config),
    };

    println!("{}", output);

    Ok(())
}

use clap::Parser;
use configuration::Configuration;
