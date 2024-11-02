use solana_program::{instruction::AccountMeta, pubkey::Pubkey};

#[repr(C)]
pub struct SolAccountMeta<'a> {
    pub pubkey_addr: &'a Pubkey,
    pub is_writable: bool,
    pub is_signer: bool,
}

impl<'a> SolAccountMeta<'a> {
    pub fn new(pubkey_addr: &'a Pubkey, is_signer: bool) -> Self {
        Self {
            pubkey_addr,
            is_writable: true,
            is_signer,
        }
    }

    pub fn new_readonly(pubkey_addr: &'a Pubkey, is_signer: bool) -> Self {
        Self {
            pubkey_addr,
            is_writable: false,
            is_signer,
        }
    }

    pub fn to_account_meta(&self) -> AccountMeta {
        AccountMeta {
            pubkey: *self.pubkey_addr,
            is_signer: self.is_signer,
            is_writable: self.is_writable,
        }
    }
}
