use crate::prelude::*;
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

pub fn sol_transfer(
    from: &Pubkey,
    to: &Pubkey,
    lamports: u64,
    accounts: &[SolAccountInfo],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let mut data: [u8; 12] = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    data[4..].copy_from_slice(&lamports.to_le_bytes());
    let accs = [
        SolAccountMeta {
            pubkey_addr: from,
            is_writable: true,
            is_signer: true,
        },
        SolAccountMeta {
            pubkey_addr: to,
            is_writable: true,
            is_signer: false,
        },
    ];
    let ix = SolInstruction::new(&solana_program::system_program::ID, &accs, &data);
    invoke_signed_unchecked(&ix, accounts, signers_seeds)
}
