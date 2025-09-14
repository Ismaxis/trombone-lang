use clap::Parser;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};
use trombone_compiler_lib::{instruction_writer::FileWriter, *};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// File to compile
    #[arg(short, long, value_name = "FILE")]
    input_file: PathBuf,

    /// Output file
    /// Default value is <input-file-no-extension>.trbc
    #[arg(short, long, value_name = "FILE")]
    output_file: Option<PathBuf>,

    /// File where compiler puts operations in human-friendly format
    #[arg(short, long, value_name = "FILE")]
    debug_output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut read_options = OpenOptions::new();
    read_options.read(true);
    let mut write_options = OpenOptions::new();
    write_options.write(true).truncate(true).create(true);

    let mut program = String::new();
    read_options
        .open(cli.input_file.clone())?
        .read_to_string(&mut program)?;

    let ops = compiler::compile_from_string(program);
    let output_file = cli
        .output_file
        .unwrap_or(cli.input_file.clone().with_extension("trbc"));
    let mut output_writer = FileWriter::new(write_options.open(output_file)?);
    output_writer.write_all(&ops)?;

    if let Some(bytecode_input) = cli.debug_output {
        let text: String = ops.iter().map(|op| format!("{:?}\n", op)).collect();
        write_options
            .open(bytecode_input)?
            .write_all(text.as_bytes())?;
    }

    Ok(())
}
