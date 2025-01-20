use crate::model::sol_account_info::SolAccountInfo;
use solana_program::{
    entrypoint::{BPF_ALIGN_OF_U128, MAX_PERMITTED_DATA_INCREASE, NON_DUP_MARKER},
    pubkey::Pubkey,
};
use std::mem::size_of;

pub const SUCCESS: u64 = 0;

pub const MAX_TX_ACCOUNTS: usize = 128;

/// TODO: Buggy on mainnet, do not use
#[macro_export]
macro_rules! nebula_entrypoint {
    ( $process_instruction:ident ) => {
        /// Program entrypoint.
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            let (program_id, accounts, instruction_data) =
                $crate::entrypoint::sol_deserialize(input);

            match $process_instruction(&program_id, &accounts, &instruction_data) {
                Ok(()) => $crate::entrypoint::SUCCESS,
                Err(error) => error.into(),
            }
        }
        solana_program::custom_heap_default!();
        solana_program::custom_panic_default!();
    };
}

pub unsafe fn sol_deserialize<'a>(input: *mut u8) -> (&'a Pubkey, Vec<SolAccountInfo>, &'a [u8]) {
    let mut offset: usize = 0;

    // Number of accounts present
    #[allow(clippy::cast_ptr_alignment)]
    let num_accounts = *(input.add(offset) as *const u64) as usize;
    offset += size_of::<u64>();

    let mut accounts = Vec::with_capacity(num_accounts);
    accounts.set_len(num_accounts);

    // Account Infos

    for i in 0..num_accounts {
        let dup_info = *(input.add(offset) as *const u8);
        offset += size_of::<u8>();
        if dup_info == NON_DUP_MARKER {
            let (account_info, new_offset) = sol_deserialize_account_info(input, offset);
            offset = new_offset;
            accounts[i] = account_info;
        } else {
            offset += 7; // padding

            // Duplicate account, clone the original
            accounts[i] = accounts[dup_info as usize];
        }
    }

    // Instruction data

    let (instruction_data, new_offset) = deserialize_instruction_data(input, offset);
    offset = new_offset;

    // Program Id

    let program_id: &Pubkey = &*(input.add(offset) as *const Pubkey);

    (program_id, accounts, instruction_data)
}

#[inline(always)]
unsafe fn sol_deserialize_account_info(
    input: *mut u8,
    mut offset: usize,
) -> (SolAccountInfo, usize) {
    #[allow(clippy::cast_ptr_alignment)]
    let is_signer = *(input.add(offset) as *const u8) != 0;
    offset += size_of::<u8>();

    #[allow(clippy::cast_ptr_alignment)]
    let is_writable = *(input.add(offset) as *const u8) != 0;
    offset += size_of::<u8>();

    #[allow(clippy::cast_ptr_alignment)]
    let executable = *(input.add(offset) as *const u8) != 0;
    offset += size_of::<u8>();

    // The original data length is stored here because these 4 bytes were
    // originally only used for padding and served as a good location to
    // track the original size of the account data in a compatible way.
    let original_data_len_offset = offset;
    offset += size_of::<u32>();

    let key = input.add(offset) as *const Pubkey;
    offset += size_of::<Pubkey>();

    let owner = input.add(offset) as *const Pubkey;
    offset += size_of::<Pubkey>();

    #[allow(clippy::cast_ptr_alignment)]
    let lamports = input.add(offset) as *mut u64;
    offset += size_of::<u64>();

    #[allow(clippy::cast_ptr_alignment)]
    let data_len = *(input.add(offset) as *const u64);
    offset += size_of::<u64>();

    // Store the original data length for detecting invalid reallocations and
    // requires that MAX_PERMITTED_DATA_LENGTH fits in a u32
    *(input.add(original_data_len_offset) as *mut u32) = data_len as u32;

    let data = input.add(offset);
    offset += data_len as usize + MAX_PERMITTED_DATA_INCREASE;
    offset += (offset as *const u8).align_offset(BPF_ALIGN_OF_U128); // padding

    #[allow(clippy::cast_ptr_alignment)]
    let rent_epoch = *(input.add(offset) as *const u64);
    offset += size_of::<u64>();

    (
        SolAccountInfo {
            key,
            is_signer,
            is_writable,
            lamports,
            data_len,
            data,
            owner,
            executable,
            rent_epoch,
        },
        offset,
    )
}

#[inline(always)]
unsafe fn deserialize_instruction_data<'a>(input: *mut u8, mut offset: usize) -> (&'a [u8], usize) {
    #[allow(clippy::cast_ptr_alignment)]
    let instruction_data_len = *(input.add(offset) as *const u64) as usize;
    offset += size_of::<u64>();

    let instruction_data = { std::slice::from_raw_parts(input.add(offset), instruction_data_len) };
    offset += instruction_data_len;

    (instruction_data, offset)
}
