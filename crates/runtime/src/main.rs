mod test;

pub use trombone_common::error::{Error, Result};

use byteorder::{ByteOrder, LittleEndian};
use clap::Parser;
use std::{
    fs,
    sync::atomic::{AtomicUsize, Ordering},
};
use trombone_common::{bytecode::Instruction, opcode};
use trombone_runner::runner::{self, ArrayOperationStream, OperationStream};

/// Virtual Machine for TromboneLang bytecode
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to input .trbc file
    path: String,
    // Jit compilation threshold
    #[arg(long, default_value_t = 8)]
    jit_threshold: usize,
    // #[clap(action)]
    // print_instructions: bool
    //
    // TODO: debug flag, step by step execution
}

fn main() -> Result<()> {
    let args = Args::parse();

    let raw = fs::read(args.path).expect("can't read input file");
    assert_eq!(
        raw.len() % 8,
        0,
        "len of executable should be multiple of 8 bytes"
    );

    let mut instructions = vec![0u64; raw.len() / 8];
    LittleEndian::read_u64_into(&raw, instructions.as_mut());

    let stream = /* TODO: buffered stream */ ArrayOperationStream::new(instructions.as_ref());
    type RunnerType<'a> = runner::Runner<
        'a,
        'a,
        ArrayOperationStream<'a>,
        std::io::BufReader<std::io::Stdin>,
        std::io::Stdout,
    >;

    let alloc = alloc::MockAllocator {
        alloc_count: AtomicUsize::new(0),
    };

    let context = trombone_jit::ExportedContext::create();
    let codegen = trombone_jit::init(&context);

    let mut runner = runner::Runner::new(
        stream,
        RunnerType::default_input(),
        RunnerType::default_output(),
        &alloc,
    );
    if args.jit_threshold == 0 {
        println!("JIT compilation is disabled");
    } else {
        println!("JIT compilation threshold: {}", args.jit_threshold);
        runner.set_codegen(codegen, args.jit_threshold);
    }

    if true == true {
        for (i, op) in runner
            .stream
            .get_next_n(runner.stream.get_instructions_len())
            .into_iter()
            .enumerate()
        {
            println!("{}: {:?}", i, op);
        }
    }

    loop {
        match runner.evaluate_next_instruction() {
            Ok(crate::runner::ReturnCode::Continue) => {}
            Ok(crate::runner::ReturnCode::Done) => {
                break;
            }
            Err(error) => {
                println!(
                    "failed at instruction: {}",
                    runner.stream.get_instruction_pointer()
                );
                return Err(error);
            }
        }
    }
    println!("Execution completed!");
    println!("Stack: {:?}", &runner.stack[..runner.sp]);
    println!("Allocations: {}", alloc.alloc_count.load(Ordering::SeqCst));
    Ok(())
}

// TODO: remove
mod alloc {
    use std::{
        alloc::{GlobalAlloc, Layout, System},
        sync::atomic::{AtomicUsize, Ordering},
    };
    pub struct MockAllocator {
        pub alloc_count: AtomicUsize,
    }

    unsafe impl GlobalAlloc for MockAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            self.alloc_count.fetch_add(1, Ordering::SeqCst);
            let ptr = unsafe { System.alloc(layout) };
            println!("Allocated {} bytes at {:?}", layout.size(), ptr);
            ptr
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            self.alloc_count.fetch_sub(1, Ordering::SeqCst);
            println!("Deallocated {} bytes at {:?}", layout.size(), ptr);
            unsafe { System.dealloc(ptr, layout) }
        }
    }
}

#[allow(dead_code)]
fn create_bytecode_file(path: &str) {
    let instructions = vec![
        Instruction::from_parts(opcode::OP_PUSH, 3).as_u64(),
        Instruction::from_parts(opcode::OP_PUSH, 2).as_u64(),
        Instruction::from_parts(opcode::OP_PUSH, 1).as_u64(),
        Instruction::from_parts(opcode::OP_ADD, 0x0).as_u64(),
        Instruction::from_parts(opcode::OP_ADD, 0x0).as_u64(),
    ];

    let mut raw = vec![0u8; instructions.len() * 8];
    LittleEndian::write_u64_into(&instructions, raw.as_mut());
    fs::write(path, raw).expect("can't write to file");
}
