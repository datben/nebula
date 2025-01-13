#[cfg(test)]
mod tests {
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        signature::Keypair,
        signer::Signer,
        transaction::Transaction,
    };
    use std::fs::File;

    #[tokio::test]
    async fn devnet() {
        let key: Vec<u8> = serde_json::from_reader(
            File::open("/Users/datben/Desktop/ben/nebula/private/devnet.json").unwrap(),
        )
        .unwrap();
        let key = Keypair::from_bytes(&key).unwrap();

        let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
        let mut accounts = vec![AccountMeta::new(key.pubkey(), true)];
        accounts.push(AccountMeta::new(key.pubkey(), false));
        accounts.push(AccountMeta::new_readonly(
            solana_program::system_program::ID,
            false,
        ));
        let ix = Instruction {
            program_id: orion_nebula::ID,
            accounts: accounts.clone(),
            data: vec![],
        };
        let b = rpc.get_latest_blockhash().await.unwrap();
        let transaction =
            Transaction::new_signed_with_payer(&[ix], Some(&key.pubkey()), &[&key], b);
        let sim = rpc.send_transaction(&transaction).await;
        println!("{:#?}", sim);
    }
}
