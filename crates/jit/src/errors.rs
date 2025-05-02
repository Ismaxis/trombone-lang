pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CompilationError,
    VerificationError,
    BuilderError(inkwell::builder::BuilderError),
}
