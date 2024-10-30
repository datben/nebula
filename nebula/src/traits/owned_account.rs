use solana_program::pubkey::Pubkey;

pub trait OwnedAccount {
    fn verify_owner(key: &Pubkey) -> Result<(), OwnedAccountError>;
}

#[derive(thiserror::Error, Debug)]
pub enum OwnedAccountError {
    #[error("Invalid owner")]
    InvalidOwner,
}

pub trait StaticOwner {
    const OWNER: &'static Pubkey;
}

impl<T: StaticOwner> OwnedAccount for T {
    fn verify_owner(key: &Pubkey) -> Result<(), OwnedAccountError> {
        if key.eq(Self::OWNER) {
            Ok(())
        } else {
            Err(OwnedAccountError::InvalidOwner)
        }
    }
}
