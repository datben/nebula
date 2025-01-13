pub trait Discriminated {
    fn verify_and_split_bytes(bytes: &[u8]) -> Result<&[u8], DiscriminatedError> {
        Ok(bytes)
    }

    fn unsafe_split_bytes(bytes: &[u8]) -> &[u8] {
        bytes
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DiscriminatedError {
    #[error("Invalid discriminant")]
    InvalidDiscriminant,
    #[error("Missing bytes")]
    MissingBytes,
}

pub trait StaticDiscriminanted {
    const DISCRIMINANT: &'static [u8];
}

impl<T> Discriminated for T
where
    T: StaticDiscriminanted,
{
    fn verify_and_split_bytes(bytes: &[u8]) -> Result<&[u8], DiscriminatedError> {
        if Self::DISCRIMINANT.len() <= bytes.len() {
            let (bytes_to_check, remaining) = (
                &bytes[..Self::DISCRIMINANT.len()],
                &bytes[Self::DISCRIMINANT.len()..],
            );
            if bytes_to_check.eq(Self::DISCRIMINANT) {
                Ok(remaining)
            } else {
                Err(DiscriminatedError::InvalidDiscriminant)
            }
        } else {
            Err(DiscriminatedError::MissingBytes)
        }
    }

    fn unsafe_split_bytes(bytes: &[u8]) -> &[u8] {
        unsafe { bytes.get_unchecked(Self::DISCRIMINANT.len()..) }
    }
}

pub trait NotDiscriminanted {}

impl<T: NotDiscriminanted> StaticDiscriminanted for T {
    const DISCRIMINANT: &'static [u8] = &[];
}
