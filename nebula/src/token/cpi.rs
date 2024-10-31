use crate::prelude::*;
use solana_program::entrypoint::ProgramResult;

use super::ASSOCIATED_TOKEN_PROGRAM;

pub fn create_associated_token_account(
    owner: &Pubkey,
    ata: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
    accounts: &[SolAccountInfo],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let accs = [
        SolAccountMeta::new(owner, true),
        SolAccountMeta::new(ata, false),
        SolAccountMeta::new_readonly(owner, false),
        SolAccountMeta::new_readonly(mint, false),
        SolAccountMeta::new_readonly(&solana_program::system_program::ID, false),
        SolAccountMeta::new_readonly(token_program, false),
    ];
    let ix = SolInstruction {
        program_id_addr: &ASSOCIATED_TOKEN_PROGRAM,
        accounts_addr: &accs,
        data_addr: &[1u8],
    };
    invoke_signed_unchecked(&ix, accounts, signers_seeds)
}

pub fn close_token_account(
    account_pubkey: &Pubkey,
    destination_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    token_program_id: &Pubkey,
    accounts: &[SolAccountInfo],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let accs = [
        SolAccountMeta::new(account_pubkey, false),
        SolAccountMeta::new(destination_pubkey, false),
        SolAccountMeta::new_readonly(owner_pubkey, true),
    ];
    let ix = SolInstruction {
        program_id_addr: token_program_id,
        accounts_addr: &accs,
        data_addr: &[9u8],
    };
    invoke_signed_unchecked(&ix, accounts, signers_seeds)
}
