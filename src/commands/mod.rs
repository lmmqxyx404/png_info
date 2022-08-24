use std::{
    io::{Read, Write},
    path::Path,
    str::FromStr,
};

use crate::{args::*, chunk::Chunk, chunk_type::ChunkType, png::Png, Error, Result};
use std::fs::OpenOptions;

fn take_png<T: AsRef<Path>>(input: T) -> Result<Png> {
    let mut file_buffer = OpenOptions::new()
        .write(true)
        .read(true)
        .open(input)
        .unwrap();
    let mut buffer = Vec::with_capacity(1000000);
    file_buffer.read_to_end(&mut buffer);
    Ok(buffer.as_slice().try_into().unwrap())
}

pub fn encode<T: AsRef<Path>>(input: T, args: EncodeArgs) -> Result<()> {
    let mut png_item = take_png(&input).unwrap();
    png_item.append_chunk(Chunk::new(
        ChunkType::from_str(&args.chunk_type).unwrap(),
        args.message.into_bytes(),
    ));
    let mut file = std::fs::File::create(input).unwrap();
    file.write_all(&png_item.as_bytes()).unwrap();
    Ok(())
}

pub fn decode<T: AsRef<Path>>(input: T, args: DecodeArgs) -> Result<()> {
    let mut png_item = take_png(&input).unwrap();
    if let Some(target_chunk) = png_item.chunk_by_type(&args.chunk_type) {
        println!(
            "Hidden message is {},in the {}",
            target_chunk.data_as_string().unwrap(),
            target_chunk.chunk_type().to_string()
        );
    } else {
        return Err(Error::from("can not find the chunk_type"));
    }
    Ok(())
}
pub fn remove<T: AsRef<Path>>(input: T, args: RemoveArgs) -> Result<()> {
    let mut png_item = take_png(&input).unwrap();

    png_item.remove_chunk(&args.chunk_type);
    let mut file = std::fs::File::create(input).unwrap();
    file.write_all(&png_item.as_bytes()).unwrap();
    Ok(())
}

pub fn print(input: &Path) -> Result<()> {
    let mut png_item = take_png(&input).unwrap();

    println!(
        "File: {},Size: {}",
        input.display(),
        png_item.as_bytes().len()
    );

    for (i, chunk) in png_item.chunks().iter().enumerate() {
        println!("{} {}", i, chunk);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_file() {
        println!("{}", 23);
    }
}
