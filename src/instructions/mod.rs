pub mod deposit;
pub use deposit::deposit;

pub mod withdraw;
pub use withdraw::withdraw;

use solana_program::program_error::ProgramError;

pub enum VaultInstruction {
    Deposit,
    Withdraw,
}

impl TryFrom<&u8> for VaultInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VaultInstruction::Deposit),
            1 => Ok(VaultInstruction::Withdraw),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
