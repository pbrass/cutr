use clap::Parser;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader };


#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cut`
/// `cutr` is a bit more permissive.
/// If you supply a bad set index in a selector, like 1-a or z-7 it will assume you
/// meant either the first or the last character for your non-numeric value.
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
    #[arg(short('s'),long("only-delimited"), default_value = "false" )]
    only_delimited: bool,

    /// Line delimiter is NULL, not newline
    #[arg(short('z'),long("zero-terminated"), default_value = "false")]
    zero_terminated: bool,

    #[clap(flatten)]
    selectors: Selectors,
}
#[derive(Debug, Parser)]
#[group(required = true, multiple = false)]
struct Selectors {
    /// Bytes to select
    #[arg(short('b'), long("bytes"), value_delimiter = ',')]
    bytes: Option<Vec<String>>,

    /// Characters to select
    #[arg(short('c'), long("characters"), value_delimiter = ',')]
    characters: Option<Vec<String>>,

    /// Fields to select
    #[arg(short('f'), long("fields"), value_delimiter = ',' )]
    fields: Option<Vec<String>>,
}


fn open_read(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(_args: Args) -> Result<()> {
    //println!("{:?}",_args);
    let line_terminator = if _args.zero_terminated { b'\0' } else { b'\n' };
    let delimiter = _args.out_delim.unwrap_or(_args.input_delim).to_string();
    let mut inner_delimiter = delimiter.clone();
    for filename in _args.files {
        match open_read(&filename) {
            Err(err) => {
                eprintln!("{filename}: Failed to open {err}, skipping");
            },
            Ok(mut h_file) => {
                let line: &mut Vec<u8> = &mut Vec::new();
                while h_file.read_until(line_terminator, line)? > 0 {
                    line.pop(); //ditch the line terminator
                    let mut fields: Vec<String> = Vec::new();
                    let mut selectors: Vec<String> = Vec::new();
                    if _args.selectors.characters.is_some() {
                        //let linestr = String::from_utf8_lossy(line.as_slice());
                        let linestr = String::from_utf8(line.to_vec()).unwrap();
                        fields = linestr.chars().map(|c| c.to_string()).collect();
                        selectors = _args.selectors.characters.clone().unwrap();
                        inner_delimiter = "".to_string();
                    }
                    else if _args.selectors.fields.is_some() {
                        selectors = _args.selectors.fields.clone().unwrap();
                        fields = line.as_slice().split(|&b| b == u8::try_from(_args.input_delim).unwrap()).map(|s| String::from_utf8_lossy(s).to_string()).collect();
                        if _args.only_delimited {
                            if fields.len() == 1  {
                                continue;
                            }
                        }
                    }
                    else if _args.selectors.bytes.is_some() {
                        selectors = _args.selectors.bytes.clone().unwrap();
                        fields = line.as_slice().iter().map(|b| String::from_utf8_lossy(&[*b]).to_string()).collect();
                        inner_delimiter = "".to_string();
                    }
                    let mut i=0;
                    for selector in selectors {
                        /*
                        if _args.complement {
                            let start:usize = selector.parse::<usize>().unwrap_or(1);
                            let end:usize = fields.len();
                            let output_string = &fields[start-1..end];
                            let output = output_string.join(&inner_delimiter);
                            print!("{}",output);
                        }
                         */
                        let default_start:usize  = 1;
                        let default_end = fields.len();
                        let mut start:usize = default_start;
                        let mut end:usize = default_end;
                        if selector.contains("-") {
                            let mut split = selector.splitn(2, '-');
                            start = split.next().unwrap().parse::<usize>().unwrap_or(default_start);
                            end = split.next().unwrap().parse::<usize>().unwrap_or(default_end);
                        }
                        else {
                            start = selector.parse::<usize>().unwrap_or(default_start);
                            end = start;
                        }
                        if start > fields.len() {continue;}
                        if end > fields.len() {end = fields.len();}
                        if i>0 {
                            print!("{}",delimiter);
                        }
                        i+=1;
                        let output_string = &fields[start-1..end];
                        let output = output_string.join(&inner_delimiter);
                        print!("{}",output);
                    }
                    print!("{}",line_terminator as char);
                    //println!("{:?}",fields);
                    line.clear();
                }
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
