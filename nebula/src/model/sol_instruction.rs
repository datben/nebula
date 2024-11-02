use solana_program::{entrypoint::ProgramResult, instruction::Instruction, pubkey::Pubkey};

use crate::prelude::{invoke_signed_unchecked, SolAccountInfo};

use super::sol_account_meta::SolAccountMeta;

#[repr(C)]
pub struct SolInstruction<'a, 'b, 'c, 'd> {
    pub program_id_addr: &'a Pubkey,
    pub accounts_addr: &'b [SolAccountMeta<'c>], // same layout as (*const SolAccountMeta, u64)
    pub data_addr: &'d [u8],                     // same layout as (*const u8, u64)
}

impl<'a, 'b, 'c, 'd> SolInstruction<'a, 'b, 'c, 'd> {
    #[inline(always)]
    pub fn new(
        program_id_addr: &'a Pubkey,
        accounts_addr: &'b [SolAccountMeta<'c>],
        data_addr: &'d [u8],
    ) -> Self {
        SolInstruction {
            program_id_addr,
            accounts_addr,
            data_addr,
        }
    }

    #[inline(always)]
    pub fn invoke(&self, accounts: &[SolAccountInfo]) -> ProgramResult {
        invoke_signed_unchecked(self, accounts, &[])
    }

    #[inline(always)]
    pub fn invoke_with_seeds(
        &self,
        accounts: &[SolAccountInfo],
        signers_seeds: &[&[&[u8]]],
    ) -> ProgramResult {
        invoke_signed_unchecked(self, accounts, signers_seeds)
    }

    pub fn to_instruction(&self) -> Instruction {
        Instruction {
            program_id: *self.program_id_addr,
            accounts: self
                .accounts_addr
                .iter()
                .map(|acc| acc.to_account_meta())
                .collect(),
            data: self.data_addr.to_vec(),
        }
    }
}
