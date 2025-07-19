use clap::Parser;
use anyhow::Result;


#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cut`
struct Args {
    /// List of input files
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Input field delimiter
    #[arg(short('d'), long("delimiter"), default_value = "\t")]
    input_delim: char,

    /// Output field delimiter, default is to use the input delimiter
    #[arg(long("output-delimiter"))]
    out_delim: Option<char>,

    /// Complement the set of bytes, characters or fields
    #[arg(long("complement"), default_value = "false")]
    complement: bool,

    /// do not print lines that do not contain a delimiter
    #[arg(short('s'),long("only-delimited"), default_value = "false")]
    only_delimited: bool,

    /// Line delimiter is NULL, not newline
    #[arg(short('z'),long("zero-terminated"), default_value = "false")]
    zero_terminated: bool,

    /// Bytes to select
    #[arg(short('b'), long("bytes"), conflicts_with("characters"), conflicts_with("fields"), value_delimiter = ',', num_args = 1..)]
    bytes: Option<Vec<String>>,

    /// Characters to select
    #[arg(short('c'), long("characters"), conflicts_with("bytes"), conflicts_with("fields"), value_delimiter = ',', num_args = 1..)]
    characters: Option<Vec<String>>,

    /// Fields to select
    #[arg(short('f'), long("fields"), conflicts_with("bytes"), conflicts_with("characters"), value_delimiter = ',', num_args = 1..)]
    fields: Option<Vec<String>>,
}


fn run(_args: Args) -> Result<()> {
    for filename in _args.files {

    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
