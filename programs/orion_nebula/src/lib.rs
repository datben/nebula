use nebula::{prelude::*, system_program::cpi::sol_transfer};
use solana_program::{entrypoint::ProgramResult, msg, pubkey::Pubkey};

solana_program::declare_id!("GeKfJNHWcueU5aNV3Af3sGUVnYn6chLfFBWoessKWkc9");

#[cfg(not(feature = "no-entrypoint"))]
nebula::nebula_entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[SolAccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    sol_transfer(
        accounts[0].key(),
        accounts[1].key(),
        1_000_000_000,
        accounts,
        &[],
    )?;
    Ok(())
}
