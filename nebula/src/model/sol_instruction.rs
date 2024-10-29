use solana_program::pubkey::Pubkey;

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
}
