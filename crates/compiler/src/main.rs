use clap::Parser;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};
use trombone_compiler_lib::{instruction_writer::FileWriter, *};

pub type Result = std::result::Result<(), CompilerError>;

#[derive(Debug)]
pub enum CompilerError {
    ReadError(Box<dyn std::error::Error>),
    WriteError(Box<dyn std::error::Error>),
    WriteDebugError(Box<dyn std::error::Error>),
}

impl CompilerError {
    fn from_io_to_read_error(error: std::io::Error) -> Self {
        Self::to_read_error(Box::new(error))
    }

    fn to_read_error(error: Box<dyn std::error::Error>) -> Self {
        Self::ReadError(error)
    }

    fn from_io_to_write_error(error: std::io::Error) -> Self {
        Self::to_write_error(Box::new(error))
    }

    fn to_write_error(error: Box<dyn std::error::Error>) -> Self {
        Self::WriteError(error)
    }

    fn from_io_to_write_debug_error(error: std::io::Error) -> Self {
        Self::to_write_debug_error(Box::new(error))
    }

    fn to_write_debug_error(error: Box<dyn std::error::Error>) -> Self {
        Self::WriteDebugError(error)
    }
}

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

fn main() -> Result {
    let cli = Cli::parse();
    let mut read_options = OpenOptions::new();
    read_options.read(true);
    let mut write_options = OpenOptions::new();
    write_options.write(true).truncate(true).create(true);

    let mut program = String::new();
    read_options
        .open(cli.input_file.clone())
        .and_then(|mut x| x.read_to_string(&mut program))
        .map_err(CompilerError::from_io_to_read_error)?;

    let ops = compiler::compile_from_string(program);
    let output_file = cli
        .output_file
        .unwrap_or(cli.input_file.clone().with_extension("trbc"));
    let mut output_writer = FileWriter::new(
        write_options
            .open(output_file)
            .map_err(CompilerError::from_io_to_write_error)?,
    );
    output_writer
        .write_all(&ops)
        .map_err(CompilerError::to_write_error)?;

    if let Some(bytecode_input) = cli.debug_output {
        let text: String = ops.iter().map(|op| format!("{:?}\n", op)).collect();
        write_options
            .open(bytecode_input)
            .and_then(|mut x| x.write_all(text.as_bytes()))
            .map_err(CompilerError::from_io_to_write_debug_error)?;
    }

    Ok(())
}
