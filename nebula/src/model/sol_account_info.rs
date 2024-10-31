use solana_program::pubkey::Pubkey;

use super::sol_account_meta::SolAccountMeta;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SolAccountInfo {
    pub key: *const Pubkey,
    pub lamports: *mut u64,
    pub data_len: u64,
    pub data: *mut u8,
    pub owner: *const Pubkey,
    pub rent_epoch: u64,
    pub is_signer: bool,
    pub is_writable: bool,
    pub executable: bool,
}

impl SolAccountInfo {
    #[inline(always)]
    pub fn key(&self) -> &Pubkey {
        unsafe { &*self.key }
    }

    #[inline(always)]
    pub fn lamports(&self) -> u64 {
        unsafe { *self.lamports }
    }

    #[inline(always)]
    pub fn owner(&self) -> &Pubkey {
        unsafe { &*self.owner }
    }

    /// Slice is valid until next realloc
    #[inline(always)]
    pub unsafe fn data_slice(&self) -> &[u8] {
        &*std::ptr::slice_from_raw_parts(self.data as *const u8, self.data_len as usize)
    }

    #[inline(always)]
    pub fn to_account_meta(&self) -> SolAccountMeta {
        SolAccountMeta {
            pubkey_addr: self.key(),
            is_writable: self.is_writable,
            is_signer: self.is_signer,
        }
    }
}
