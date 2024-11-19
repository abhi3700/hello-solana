use anchor_client::{solana_sdk::signature::read_keypair_file, Client, Cluster};
use solana_sdk::commitment_config::CommitmentConfig;
use std::rc::Rc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Wallet and cluster params.
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    // For local testnet. Comment/uncomment.
    let url = Cluster::Custom(
        "http://localhost:8899".to_string(),
        "ws://127.0.0.1:8900".to_string(),
    );

    let url = Cluster::Devnet;

    // Client.
    let payer = Rc::new(payer);
    let anchor_client =
        Client::new_with_options(url.clone(), payer.clone(), CommitmentConfig::processed());

    // Create program
    let foo = anchor_client.program(foo::ID)?;
    let foo_signature = foo
        .request()
        .accounts(foo::accounts::Initialize {})
        .args(foo::instruction::Initialize {})
        .signer(&payer)
        .send()
        .await?;
    println!("Tx signature: {:?}", foo_signature);

    Ok(())
}
