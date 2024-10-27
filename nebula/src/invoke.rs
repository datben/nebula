use crate::c_struct::{SolAccountInfo, SolInstruction};
use solana_program::entrypoint::ProgramResult;

type SolInvokeSignedC = fn(
    *const u8, // instruction pointer
    *const u8, // account_infos pointer
    u64,       // length of account_infos
    *const u8, // signers_seeds pointer
    u64,       // length of signers_seeds
) -> u64;

pub fn invoke_signed_unchecked(
    instruction: &SolInstruction,
    account_infos: &[SolAccountInfo],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let result = unsafe {
        // https://github.com/anza-xyz/agave/blob/master/sdk/sbf/c/inc/sol/cpi.h#L83
        let invoke: SolInvokeSignedC = std::mem::transmute(2720767109u64);
        invoke(
            instruction as *const _ as *const u8,
            account_infos as *const _ as *const u8,
            account_infos.len() as u64,
            signers_seeds as *const _ as *const u8,
            signers_seeds.len() as u64,
        )
    };
    match result {
        crate::entrypoint::SUCCESS => Ok(()),
        _ => Err(result.into()),
    }
}
