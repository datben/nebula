#[cfg(test)]
mod tests {
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        signature::Keypair,
        signer::Signer,
    };

    use crate::fixture::NebulaFixture;

    #[tokio::test]
    async fn simple() {
        let fixture = NebulaFixture::new().await;
        let kp = Keypair::new();
        let ix = Instruction {
            program_id: fixture.nebula_key,
            accounts: vec![AccountMeta::new(fixture.context.payer.pubkey(), true)],
            data: vec![],
        };
    }
}
