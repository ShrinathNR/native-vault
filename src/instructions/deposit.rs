use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke,
    program_error::ProgramError, pubkey::Pubkey, system_instruction,
};

pub fn deposit(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    let [user, vault, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    let vault_pda = Pubkey::find_program_address(&[b"vault", user.key.as_ref()], &crate::ID);

    if vault_pda.0.ne(vault.key) {
        return Err(ProgramError::InvalidAccountData);
    }

    let transfer_ix = system_instruction::transfer(user.key, vault.key, lamports);

    invoke(&transfer_ix, &[user.clone(), vault.clone()])
}
