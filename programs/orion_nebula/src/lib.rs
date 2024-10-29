use nebula::prelude::*;
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

#[cfg(not(feature = "no-entrypoint"))]
nebula::nebula_entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[SolAccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
