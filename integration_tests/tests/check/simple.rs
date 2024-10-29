#[cfg(test)]
mod tests {
    use std::time::Duration;

    use solana_sdk::{
        commitment_config::CommitmentLevel,
        compute_budget::ComputeBudgetInstruction,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signer::Signer,
        transaction::Transaction,
    };

    use crate::fixture::NebulaFixture;

    #[tokio::test]
    async fn simple() {
        let mut fixture = NebulaFixture::new().await;
        let mut accounts = vec![AccountMeta::new(fixture.context.payer.pubkey(), true)];
        accounts.push(AccountMeta::new(Pubkey::new_unique(), false));
        accounts.push(AccountMeta::new_readonly(
            solana_program::system_program::ID,
            false,
        ));
        let ix = Instruction {
            program_id: fixture.nebula_key,
            accounts: accounts.clone(),
            data: vec![],
        };
        let b = fixture.context.get_new_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[
                ComputeBudgetInstruction::set_compute_unit_limit(1_400_000),
                ix,
            ],
            Some(&fixture.context.payer.pubkey()),
            &[&fixture.context.payer],
            b,
        );
        println!(
            "{}",
            fixture
                .context
                .banks_client
                .get_balance_with_commitment(
                    fixture.context.payer.pubkey(),
                    CommitmentLevel::Processed
                )
                .await
                .unwrap()
        );
        let sim = fixture
            .context
            .banks_client
            .send_transaction(transaction)
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;

        println!(
            "{}",
            fixture
                .context
                .banks_client
                .get_balance_with_commitment(
                    fixture.context.payer.pubkey(),
                    CommitmentLevel::Processed
                )
                .await
                .unwrap()
        );
    }
}
