use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey;
use solana_program::{account_info::AccountInfo, entrypoint, pubkey::Pubkey};
mod instructions;
use instructions::VaultInstruction;

#[cfg(test)]
mod tests;

entrypoint!(process_instruction);

const ID: Pubkey = pubkey!("9HFegTZnvebYjf9kSa6k3WBm93hRfogWB5B1goUrq1oL");

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

    let lamports = ix_data
        .get(..8)
        .and_then(|bytes| bytes.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    match VaultInstruction::try_from(discriminator)? {
        VaultInstruction::Deposit => instructions::deposit(accounts, lamports),
        VaultInstruction::Withdraw => instructions::withdraw(accounts, lamports),
    }
}
