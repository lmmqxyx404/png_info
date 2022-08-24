use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, StructOpt)]

pub struct Opt {
    /// file path
    #[structopt(parse(from_os_str), name = "FilePath")]
    pub input: PathBuf,
    /// must indicate the macro because
    #[structopt(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, StructOpt)]
pub struct EncodeArgs {
    pub message: String,
    pub chunk_type: String,
}

#[derive(Debug, StructOpt)]
pub struct DecodeArgs {
    pub chunk_type: String,
}
#[derive(Debug, StructOpt)]
pub struct RemoveArgs {
    pub chunk_type: String,
}

#[derive(Debug, StructOpt)]
pub struct PrintArgs {}
