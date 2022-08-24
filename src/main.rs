use std::path::PathBuf;
use structopt::StructOpt;

use png_info::args::{Commands::*, EncodeArgs, Opt};
use png_info::commands::*;
use png_info::Result;

fn main() {
    run();
}

fn run() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt {
            input,
            commands: Encode(args),
        } => encode(input, args)?,
        Opt {
            input,
            commands: Decode(args),
        } => decode(input, args)?,
        Opt {
            input,
            commands: Remove(args),
        } => remove(input, args)?,
        Opt {
            input,
            commands: Print(_),
        } => print(&input)?,
    }
    Ok(())
}
