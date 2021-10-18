use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    //Invalid Instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

// The reason we do this conversion in the first place is that the
//entrypoint returns a `Result` of either nothing or a `ProgramError`
impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}