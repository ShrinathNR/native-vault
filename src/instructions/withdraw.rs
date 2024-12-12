use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed,
    program_error::ProgramError, pubkey::Pubkey, system_instruction,
};

pub fn withdraw(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    let [user, vault, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let vault_pda = Pubkey::find_program_address(&[b"vault", user.key.as_ref()], &crate::ID);

    if vault_pda.0.ne(vault.key) {
        return Err(ProgramError::InvalidAccountData);
    }

    let transfer_ix = system_instruction::transfer(vault.key, user.key, lamports);

    invoke_signed(
        &transfer_ix,
        &[user.clone(), vault.clone()],
        &[&[b"vault", &user.key.to_bytes(), &[vault_pda.1]]],
    )
}
