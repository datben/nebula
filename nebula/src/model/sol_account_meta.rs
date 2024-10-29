use solana_program::pubkey::Pubkey;

#[repr(C)]
pub struct SolAccountMeta<'a> {
    pub pubkey_addr: &'a Pubkey,
    pub is_writable: bool,
    pub is_signer: bool,
}
