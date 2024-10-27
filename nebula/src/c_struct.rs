use solana_program::pubkey::Pubkey;

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

    /// Slice is valid until next realloc
    #[inline(always)]
    pub unsafe fn data_slice(&self) -> &[u8] {
        std::slice::from_raw_parts(self.data as *const u8, self.data_len as usize)
    }
}

#[repr(C)]
pub struct SolAccountMeta<'a> {
    pub pubkey: &'a Pubkey,
    pub is_writable: bool,
    pub is_signer: bool,
}

#[repr(C)]
pub struct SolInstruction<'a> {
    program_id: &'a Pubkey,
    accounts: *const SolAccountMeta<'a>,
    account_len: u64,
    data: *const u8,
    data_len: u64,
}

impl<'a> SolInstruction<'a> {
    pub fn new(program_id: &'a Pubkey, accounts: &'a [SolAccountMeta<'a>], data: &'a [u8]) -> Self {
        SolInstruction {
            program_id,
            accounts: accounts.as_ptr(),
            account_len: accounts.len() as u64,
            data: data.as_ptr(),
            data_len: data.len() as u64,
        }
    }
}
