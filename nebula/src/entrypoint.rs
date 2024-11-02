use solana_program::{
    entrypoint::{BPF_ALIGN_OF_U128, MAX_PERMITTED_DATA_INCREASE, NON_DUP_MARKER},
    pubkey::Pubkey,
};

use crate::model::sol_account_info::SolAccountInfo;

pub const SUCCESS: u64 = 0;

#[macro_export]
macro_rules! nebula_entrypoint {
    ($process_instruction:ident) => {
        /// # Safety
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            let (program_id, accounts, instruction_data) =
                unsafe { $crate::entrypoint::sol_deserialize(input) };
            match $process_instruction(&program_id, &accounts, &instruction_data) {
                Ok(()) => $crate::entrypoint::SUCCESS,
                Err(error) => error.into(),
            }
        }
        solana_program::custom_heap_default!();
        solana_program::custom_panic_default!();
    };
}

pub unsafe fn sol_deserialize<'a>(input: *const u8) -> (&'a Pubkey, Vec<SolAccountInfo>, &'a [u8]) {
    use std::mem::size_of;
    let mut offset: usize = 0;
    let num_accounts = *(input.add(offset) as *const u64) as usize;
    offset += size_of::<u64>();

    let mut accounts = Vec::<SolAccountInfo>::with_capacity(num_accounts);
    accounts.set_len(num_accounts);
    for i in 0..num_accounts {
        let dup_info = *(input.add(offset) as *const u8);
        offset += size_of::<u8>();
        if dup_info == NON_DUP_MARKER {
            accounts[i].is_signer = *(input.add(offset) as *const u8) != 0;
            offset += size_of::<u8>();

            accounts[i].is_writable = *(input.add(offset) as *const u8) != 0;
            offset += size_of::<u8>();

            accounts[i].executable = *(input.add(offset) as *const u8) != 0;
            offset += size_of::<u8>();

            // padding
            offset += size_of::<u32>();

            // Assign pointers
            accounts[i].key = input.add(offset) as *const Pubkey;
            offset += size_of::<Pubkey>();

            accounts[i].owner = input.add(offset) as *const Pubkey;
            offset += size_of::<Pubkey>();

            accounts[i].lamports = input.add(offset) as *mut u64;
            offset += size_of::<u64>();

            let data_len = *(input.add(offset) as *const u64) as u64;
            offset += size_of::<u64>();
            accounts[i].data_len = data_len;

            accounts[i].data = input.add(offset) as *mut u8;

            offset += data_len as usize + MAX_PERMITTED_DATA_INCREASE;
            offset += (offset as *const u8).align_offset(BPF_ALIGN_OF_U128); // padding

            accounts[i].rent_epoch = *(input as *const u64);
            offset += size_of::<u64>();
        } else {
            offset += 7;
            accounts[i] = accounts[dup_info as usize];
        }
    }

    let instruction_data_len = *(input.add(offset) as *const u64) as usize;
    offset += size_of::<u64>();

    let instruction_data = { std::slice::from_raw_parts(input.add(offset), instruction_data_len) };
    offset += instruction_data_len;

    let program_id = &*(input.add(offset) as *const Pubkey);

    (program_id, accounts, instruction_data)
}
