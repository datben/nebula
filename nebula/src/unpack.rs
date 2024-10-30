use solana_program::pubkey::Pubkey;

use crate::traits::wrapper::ToSome;

pub const U64_BYTES: usize = 8;
pub const PUBKEY_BYTES: usize = 32;

#[inline(always)]
const fn as_array<const LEN: usize>(slice: &[u8]) -> &[u8; LEN] {
    unsafe { &*(slice.as_ptr() as *const [u8; LEN]) }
}

#[inline(always)]
fn unpack_u64(bytes: &[u8], offset: usize) -> Option<u64> {
    u64::from_le_bytes(*as_array(bytes.get(offset..offset + U64_BYTES)?)).some()
}

#[inline(always)]
fn unpack_pubkey(bytes: &[u8], offset: usize) -> Option<Pubkey> {
    Pubkey::new_from_array(*as_array(bytes.get(offset..offset + PUBKEY_BYTES)?)).some()
}

#[inline(always)]
fn unpack_pubkey_ref(bytes: &[u8], offset: usize) -> Option<&Pubkey> {
    // always safe because align of Pubkey == 0x1
    unsafe { &*(bytes.get(offset..offset + PUBKEY_BYTES)?.as_ptr() as *const Pubkey) }.some()
}

pub const SPL_TOKEN_ACCOUNT_MINT_OFFSET: usize = 0;
pub const SPL_TOKEN_ACCOUNT_OWNER_OFFSET: usize = SPL_TOKEN_ACCOUNT_MINT_OFFSET + PUBKEY_BYTES;
pub const SPL_TOKEN_ACCOUNT_AMOUNT_OFFSET: usize = SPL_TOKEN_ACCOUNT_OWNER_OFFSET + PUBKEY_BYTES;

#[inline(always)]
pub fn unpack_token_account_amount(bytes: &[u8]) -> Option<u64> {
    unpack_u64(bytes, SPL_TOKEN_ACCOUNT_AMOUNT_OFFSET)
}

#[inline(always)]
pub fn unpack_token_account_mint(bytes: &[u8]) -> Option<Pubkey> {
    unpack_pubkey(bytes, SPL_TOKEN_ACCOUNT_MINT_OFFSET)
}

#[inline(always)]
pub fn unpack_token_account_mint_ref(bytes: &[u8]) -> Option<&Pubkey> {
    unpack_pubkey_ref(bytes, SPL_TOKEN_ACCOUNT_MINT_OFFSET)
}

pub const SPL_TOKEN_MINT_DECIMAL_OFFSET: usize = 36 + 8;

#[inline(always)]
pub fn unpack_mint_decimal(bytes: &[u8]) -> Option<u8> {
    bytes.get(SPL_TOKEN_MINT_DECIMAL_OFFSET).copied()
}
