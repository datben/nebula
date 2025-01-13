use std::fs::File;

use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};

#[tokio::main]
async fn main() {
    let key: Vec<u8> = serde_json::from_reader(
        File::open("/Users/datben/Desktop/ben/nebula/private/mainnet.json").unwrap(),
    )
    .unwrap();
    let key = Keypair::from_bytes(&key).unwrap();

    let rpc = solana_client::nonblocking::rpc_client::RpcClient::new(
        "https://api.mainnet-beta.solana.com".to_string(),
    );
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
    let transaction = Transaction::new_signed_with_payer(
        &[
            ComputeBudgetInstruction::set_compute_unit_limit(10_000),
            ComputeBudgetInstruction::set_compute_unit_price(200_000),
            ix,
        ],
        Some(&key.pubkey()),
        &[&key],
        b,
    );
    let sim = rpc.send_transaction(&transaction).await;
    println!("{sim:?}")
}
