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

impl<'a> SolAccountInfosToMeta<'a> for [SolAccountInfo] {
    type Output = Vec<SolAccountMeta<'a>>;

    fn to_account_metas(&'a self) -> Self::Output {
        self.iter().map(|acc| acc.to_account_meta()).collect()
    }
}

pub trait SolAccountInfosToMetaWithSkip<'a> {
    fn to_account_metas_with_skip(&'a self, skip: &[usize]) -> Vec<SolAccountMeta<'a>>;
}

impl<'a> SolAccountInfosToMetaWithSkip<'a> for [SolAccountInfo] {
    fn to_account_metas_with_skip(&'a self, skip: &[usize]) -> Vec<SolAccountMeta<'a>> {
        self.iter()
            .enumerate()
            .filter(|(i, _)| !skip.contains(i))
            .map(|acc| acc.1.to_account_meta())
            .collect()
    }
}
