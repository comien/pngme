use std::path::PathBuf;

pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

pub struct EncodeArgs {
    // Write me!
    pub path: PathBuf,
    pub chunk_type: String,
    pub data: String,
}

pub struct DecodeArgs {
    // Write me!
    pub path: PathBuf,
    pub chunk_type: String,
}

pub struct RemoveArgs {
    // Write me!
    pub path: PathBuf,
    pub chunk_type: String,
}

pub struct PrintArgs {
    // Write me!
    pub path: PathBuf,
}