use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Commands {
    /// hide the message to the specific chunk
    Encode(EncodeArgs),
    /// decode the info from specific chunk
    Decode(DecodeArgs),
    /// delete the info from the chunk
    Remove(RemoveArgs),
    ///  print the chunk info from the file
    Print(PrintArgs),
}

/// Command line tools to help you hide messages in a png file
#[derive(Debug, StructOpt)]
#[structopt(name = "png_info")]
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
