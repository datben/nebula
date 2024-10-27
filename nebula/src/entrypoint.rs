use solana_program::{
    entrypoint::{BPF_ALIGN_OF_U128, MAX_PERMITTED_DATA_INCREASE, NON_DUP_MARKER},
    pubkey::Pubkey,
};

use crate::c_struct::SolAccountInfo;

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
    let accounts_len = *(input as *const u64) as usize;
    let mut accounts = Vec::<SolAccountInfo>::with_capacity(accounts_len);
    accounts.set_len(accounts_len);
    let mut input = input.add(std::mem::size_of::<u64>());
    for i in 0..accounts_len {
        let dup_info = *input as u8;
        input = input.add(std::mem::size_of::<u8>());
        if dup_info == NON_DUP_MARKER {
            // Assign fields from input
            accounts[i].is_signer = *input != 0;
            input = input.add(std::mem::size_of::<u8>());

            accounts[i].is_writable = *input != 0;
            input = input.add(std::mem::size_of::<u8>());

            accounts[i].executable = *input != 0;
            input = input.add(std::mem::size_of::<u8>());

            input = input.add(4); // padding

            // Assign pointers
            accounts[i].key = input as *const Pubkey;
            input = input.add(std::mem::size_of::<Pubkey>());

            accounts[i].owner = input as *const Pubkey;
            input = input.add(std::mem::size_of::<Pubkey>());

            accounts[i].lamports = input as *mut u64;
            input = input.add(std::mem::size_of::<u64>());

            // Account data
            let data_len = *(input as *const u64);
            input = input.add(std::mem::size_of::<u64>());
            accounts[i].data_len = data_len;

            accounts[i].data = input as *mut u8;
            input = input.add(data_len as usize + MAX_PERMITTED_DATA_INCREASE);
            input = input.add(input.align_offset(BPF_ALIGN_OF_U128));

            // Rent epoch
            accounts[i].rent_epoch = *(input as *const u64);
            input = input.add(std::mem::size_of::<u64>());
        } else {
            // Duplicate info handling
            accounts[i] = accounts[dup_info as usize];
        }
    }

    let data_len = *(input as *const u64);
    input = input.add(std::mem::size_of::<u64>());
    let data_ref = input as *const u8;

    let program_id = input as *const Pubkey;

    (
        &*program_id,
        accounts,
        std::slice::from_raw_parts(data_ref, data_len as usize),
    )
}
