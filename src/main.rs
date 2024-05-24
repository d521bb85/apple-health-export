use clap::{Parser, Subcommand};
mod commands;

#[derive(Parser, Debug)]
struct Entry {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Scan(commands::scan::Input),
    Convert(commands::convert::Input),
}

fn main() {
    let entry = Entry::parse();
    match &entry.command {
        Command::Scan(input) => commands::scan::exec(input),
        Command::Convert(input) => commands::convert::exec(input),
    }
}
