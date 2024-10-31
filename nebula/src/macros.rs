#[macro_export]
macro_rules! impl_static_account {
    ($type:ty, $discriminant:expr, $owner:expr) => {
        impl StaticDiscriminanted for $type {
            const DISCRIMINANT: &'static [u8] = $discriminant;
        }

        impl StaticOwner for $type {
            const OWNER: Pubkey = $owner;
        }
    };
}
