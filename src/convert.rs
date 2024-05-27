use clap::Args;
use std::io;

#[derive(Args, Debug)]
pub struct Input {}

pub fn exec(_input: &Input) -> Result<(), io::Error> {
    Ok(())
}
