pub mod entrypoint;
pub mod model;
pub mod syscall;
pub mod system_program;
pub mod traits;
pub mod unpack;

pub mod prelude {
    pub use crate::model::{sol_account_info::*, sol_account_meta::*, sol_instruction::*};
    pub use crate::syscall::invoke::invoke_signed_unchecked;
    pub use solana_program::pubkey::Pubkey;
}
