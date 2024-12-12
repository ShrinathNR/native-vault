use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey;
use solana_program::{account_info::AccountInfo, entrypoint, pubkey::Pubkey};
mod instructions;
use instructions::VaultInstruction;

entrypoint!(process_instruction);

const ID: Pubkey = pubkey!("tjGXp9aPM7WNcMpBqAtDVVkZMjJAACvW1BE9ADgPEeV");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&ID) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (discriminator, ix_data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match VaultInstruction::try_from(discriminator)? {
        VaultInstruction::Deposit => instructions::deposit(accounts, ix_data),
        VaultInstruction::Withdraw => instructions::withdraw(accounts, ix_data),
    }
}
