use bytemuck::{Pod, PodCastError};

use super::discriminant::Discriminated;

pub trait AccountReader {
    fn load_as_ref_unchecked<T: Pod>(&self) -> Result<&T, AccountReaderError> {}
}

#[derive(thiserror::Error, Debug)]
pub enum AccountReaderError {
    #[error("Failed to cast account : {0}")]
    PodCast(PodCastError),
}

impl From<PodCastError> for AccountReaderError {
    fn from(value: PodCastError) -> Self {
        Self::PodCast(value)
    }
}
