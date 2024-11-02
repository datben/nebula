use crate::traits::wrapper::ToSome;
use crate::unpack::unpack_mint_decimal;
use crate::{
    prelude::SolAccountInfo,
    unpack::{unpack_token_account_amount, unpack_token_account_mint_ref},
};
use borsh::BorshDeserialize;
use bytemuck::{Pod, PodCastError};
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use super::{
    discriminant::{Discriminated, DiscriminatedError},
    owned_account::{OwnedAccount, OwnedAccountError},
};

pub trait AccountReader {
    fn key(&self) -> &Pubkey;

    fn data_ref(&self) -> &[u8];

    fn owner(&self) -> &Pubkey;

    fn unpack_token_account_amount(&self) -> Result<u64, AccountReaderError> {
        unpack_token_account_amount(self.data_ref())
            .ok_or_else(|| AccountReaderError::InvalidDataLen)
    }

    fn unpack_mint_account_decimal(&self) -> Result<u8, AccountReaderError> {
        unpack_mint_decimal(self.data_ref()).ok_or_else(|| AccountReaderError::InvalidDataLen)
    }

    fn load_token_account_mint_ref(&self) -> Result<&Pubkey, AccountReaderError> {
        unpack_token_account_mint_ref(self.data_ref())
            .ok_or_else(|| AccountReaderError::InvalidDataLen)
    }

    fn load_as_ref<T: Pod + Discriminated + OwnedAccount>(&self) -> Result<&T, AccountReaderError> {
        T::verify_owner(self.owner())?;
        let raw_data = self.data_ref();
        let data = T::verify_and_split_bytes(raw_data)?
            .get(..std::mem::size_of::<T>())
            .ok_or_else(|| AccountReaderError::InvalidDataLen)?;
        Ok(bytemuck::try_from_bytes(data)?)
    }

    fn load_as_ref_maybe_uninit<T: Pod + Discriminated + OwnedAccount>(
        &self,
    ) -> Result<Option<&T>, AccountReaderError> {
        if self.owner().ne(&solana_program::system_program::ID) {
            self.load_as_ref().map(|res: &T| res.some())
        } else {
            if self.data_ref().len() == 0 {
                Ok(None)
            } else {
                Err(AccountReaderError::InvalidDataLen)
            }
        }
    }

    fn load_as_ref_at<T: Pod>(&self, offset: usize) -> Result<&T, AccountReaderError> {
        let data = self
            .data_ref()
            .get(offset..offset + std::mem::size_of::<T>())
            .ok_or_else(|| AccountReaderError::InvalidDataLen)?;
        Ok(bytemuck::try_from_bytes(data)?)
    }

    fn deserialize<T: BorshDeserialize + Discriminated + OwnedAccount>(
        &self,
    ) -> Result<T, AccountReaderError> {
        T::verify_owner(self.owner())?;
        let raw_data = self.data_ref();
        let data = T::verify_and_split_bytes(raw_data)?;
        Ok(T::deserialize(&mut &*data)?)
    }

    fn deserialize_at<T: BorshDeserialize>(&self, offset: usize) -> Result<T, AccountReaderError> {
        let data = self
            .data_ref()
            .get(offset..)
            .ok_or_else(|| AccountReaderError::InvalidDataLen)?;
        Ok(T::deserialize(&mut &*data)?)
    }
}

impl AccountReader for SolAccountInfo {
    #[inline(always)]
    fn data_ref(&self) -> &[u8] {
        unsafe { self.data_slice() }
    }

    #[inline(always)]
    fn owner(&self) -> &Pubkey {
        self.owner()
    }

    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.key()
    }
}

impl AccountReader for AccountInfo<'_> {
    #[inline(always)]
    fn data_ref(&self) -> &[u8] {
        unsafe { &*self.data.as_ptr() }
    }

    #[inline(always)]
    fn owner(&self) -> &Pubkey {
        self.owner
    }

    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.key
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AccountReaderError {
    #[error("Failed to cast account : {0}")]
    PodCast(PodCastError),
    #[error("Failed to verify discriminant : {0}")]
    Discriminant(#[from] DiscriminatedError),
    #[error("{0}")]
    InvalidOwner(#[from] OwnedAccountError),
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("Data slice is too short")]
    InvalidDataLen,
    #[error("ProgramError : {0}")]
    ProgramError(#[from] ProgramError),
}

impl From<PodCastError> for AccountReaderError {
    fn from(value: PodCastError) -> Self {
        Self::PodCast(value)
    }
}
