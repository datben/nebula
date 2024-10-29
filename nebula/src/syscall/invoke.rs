use solana_program::entrypoint::ProgramResult;

use crate::model::{sol_account_info::SolAccountInfo, sol_instruction::SolInstruction};

#[allow(unused_variables)]
pub fn invoke_signed_unchecked(
    instruction: &SolInstruction,
    account_infos: &[SolAccountInfo],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    #![allow(unexpected_cfgs)]
    #[cfg(target_os = "solana")]
    {
        let result = unsafe {
            solana_program::syscalls::sol_invoke_signed_c(
                instruction as *const _ as *const _,
                account_infos.as_ptr() as *const _,
                account_infos.len() as u64,
                signers_seeds.as_ptr() as *const _,
                signers_seeds.len() as u64,
            )
        };
        match result {
            crate::entrypoint::SUCCESS => {}
            _ => return Err(result.into()),
        };
    }
    Ok(())
}
