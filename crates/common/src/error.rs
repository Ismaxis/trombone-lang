pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

// use derive_more::From;
//
// #[derive(Debug, From)]
// pub enum Error {
//     #[from]
//     LLVM(inkwell::support::LLVMString),

//     #[from]
//     Custom(String),
// }
