#[macro_export]
macro_rules! impl_static_account {
    ($type:ty, $discriminant:expr, $owner:expr) => {
        impl $crate::traits::discriminant::StaticDiscriminated for $type {
            const DISCRIMINANT: &'static [u8] = $discriminant;
        }

        impl $crate::traits::owned_account::StaticOwner for $type {
            const OWNER: &'static $crate::prelude::Pubkey = $owner;
        }
    };
}

#[macro_export]
macro_rules! impl_anchor_account {
    ($type:ty, $owner:expr) => {
        impl $crate::traits::discriminant::StaticDiscriminated for $type {
            const DISCRIMINANT: &'static [u8] =
                &$crate::const_utils::anchor_account_sighash(stringify!($type));
        }

        impl $crate::traits::owned_account::StaticOwner for $type {
            const OWNER: &'static $crate::prelude::Pubkey = $owner;
        }
    };
}

#[macro_export]
macro_rules! const_array_concat {
    ($array:expr,$N:expr) => {{
        const RESULT: (usize, [u8; $N]) = $crate::const_utils::const_concat($array);
        static_assertions::const_assert_eq!(RESULT.0, $N);
        RESULT.1
    }};
}

#[cfg(test)]
mod test {
    use solana_program::pubkey::Pubkey;

    use crate::{
        const_utils::anchor_account_sighash, traits::discriminant::StaticDiscriminated,
        traits::owned_account::StaticOwner,
    };

    #[test]
    fn test_static_account() {
        struct Whirlpool;
        impl_anchor_account!(Whirlpool, &Pubkey::new_from_array([1u8; 32]));
        assert_eq!(Whirlpool::DISCRIMINANT, anchor_account_sighash("Whirlpool"));
        assert_eq!(Whirlpool::OWNER, &Pubkey::new_from_array([1u8; 32]));
    }

    #[test]
    fn test_const_array_concat() {
        const FIVE_SIX: [u8; 2] = [5, 6];
        let result = const_array_concat!([&[1u8, 2, 3, 4u8], &FIVE_SIX, &[7u8, 8, 9]], 9);
        assert_eq!(result, [1u8, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
