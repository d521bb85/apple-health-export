use clap::{Parser, Subcommand};
mod convert;
mod records;
mod scan;
mod value_parsers;

#[derive(Parser, Debug)]
struct Entry {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Scan(scan::Input),
    Convert(convert::Input),
}

fn main() {
    let entry = Entry::parse();
    match &entry.command {
        Command::Scan(input) => scan::exec(input),
        Command::Convert(input) => convert::exec(input),
    };
}
