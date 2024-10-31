#[macro_export]
macro_rules! impl_static_account {
    ($type:ty, $discriminant:expr, $owner:expr) => {
        impl $crate::traits::discriminant::StaticDiscriminanted for $type {
            const DISCRIMINANT: &'static [u8] = $discriminant;
        }

        impl $crate::traits::owned_account::StaticOwner for $type {
            const OWNER: Pubkey = $owner;
        }
    };
}
