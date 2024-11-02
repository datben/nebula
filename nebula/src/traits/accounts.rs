use solana_program::{account_info::AccountInfo, instruction::AccountMeta};

use crate::prelude::{SolAccountInfo, SolAccountMeta};

pub trait SolAccountInfosToMeta<'a> {
    type Output;
    fn to_account_metas(&'a self) -> Self::Output;
}

impl<'a, const LEN: usize> SolAccountInfosToMeta<'a> for [SolAccountInfo; LEN] {
    type Output = [SolAccountMeta<'a>; LEN];

    fn to_account_metas(&'a self) -> Self::Output {
        std::array::from_fn(|i| self[i].to_account_meta())
    }
}

impl<'a> SolAccountInfosToMeta<'a> for [AccountInfo<'a>] {
    type Output = Vec<AccountMeta>;

    fn to_account_metas(&'a self) -> Self::Output {
        self.iter()
            .map(|acc| AccountMeta {
                pubkey: *acc.key,
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect()
    }
}

impl<'a> SolAccountInfosToMeta<'a> for [SolAccountInfo] {
    type Output = Vec<SolAccountMeta<'a>>;

    fn to_account_metas(&'a self) -> Self::Output {
        self.iter().map(|acc| acc.to_account_meta()).collect()
    }
}

pub trait SolAccountInfosToMetaWithSkip<'a> {
    type Output;
    fn to_account_metas_with_skip(&'a self, skip: &[usize]) -> Self::Output;
}

impl<'a> SolAccountInfosToMetaWithSkip<'a> for [SolAccountInfo] {
    type Output = Vec<SolAccountMeta<'a>>;
    fn to_account_metas_with_skip(&'a self, skip: &[usize]) -> Self::Output {
        self.iter()
            .enumerate()
            .filter(|(i, _)| !skip.contains(i))
            .map(|acc| acc.1.to_account_meta())
            .collect()
    }
}

impl<'a> SolAccountInfosToMetaWithSkip<'a> for [AccountInfo<'a>] {
    type Output = Vec<AccountMeta>;
    fn to_account_metas_with_skip(&'a self, skip: &[usize]) -> Self::Output {
        self.iter()
            .enumerate()
            .filter(|(i, _)| !skip.contains(i))
            .map(|acc| AccountMeta {
                pubkey: *acc.1.key,
                is_signer: acc.1.is_signer,
                is_writable: acc.1.is_writable,
            })
            .collect()
    }
}
