use thiserror::Error;
#[derive(Debug, Error)]
pub enum UniformError {
    #[error("Uniform '{name}' not found in program {program_id}")]
    NotFound { name: String, program_id: u32 },
    #[error("Failed to convert uniform name to CString: {0}")]
    NulError(#[from] std::ffi::NulError),
}
