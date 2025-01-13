#![allow(unexpected_cfgs)]

#[inline]
pub unsafe fn sol_memcpy_src(dst: &mut [u8], src: &[u8]) {
    #[cfg(target_os = "solana")]
    solana_program::syscalls::sol_memcpy_(dst.as_mut_ptr(), src.as_ptr(), src.len() as u64);

    #[cfg(not(target_os = "solana"))]
    core::hint::black_box((dst, src, src.len()));
}
