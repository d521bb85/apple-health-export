use crate::records::Reader;
use crate::value_parsers;
use clap::Args;
use std::io;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct Input {
    #[arg(value_parser = value_parsers::to_path_buf)]
    source: PathBuf,
}

pub fn exec(input: &Input) -> Result<(), io::Error> {
    let mut reader = Reader::new(&input.source)?;
    loop {
        match reader.read_next_record() {
            Ok(Some(record)) => {
                println!("{:?}", record);
            }
            Ok(None) => break,
            Err(_) => panic!("Something went wrong!"),
        }
    }
    Ok(())
}
