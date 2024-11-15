//! Using "Program A" program to test CPI
//!
//! NOTE: Make sure the program is deployed/available on the network being used.
//!
//! -
//! -
//!
//! TODO: Write code to execute concurrently. First 2.

use anchor_client::{
    anchor_lang::system_program, solana_sdk::signature::read_keypair_file, Client, Cluster,
};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Wallet and cluster params
    let alice = read_keypair_file(&*shellexpand::tilde("~/.config/solana/alice.json"))
        .expect("Example requires a keypair file");
    // For local testnet. Comment/uncomment.
    let cluster = Cluster::Custom(
        "http://localhost:8899".to_string(),
        "ws://127.0.0.1:8900".to_string(),
    );

    // let url = Cluster::Devnet;

    // Client
    let alice = Arc::new(alice);
    let client = Client::new_with_options(cluster, alice.clone(), CommitmentConfig::processed());

    // get instance of Program A
    let program_a = client
        .program(program_a::ID)
        .expect("Program A doesn't exist");

    let (pda_account, _bump) =
        Pubkey::find_program_address(&[b"abhi", alice.pubkey().as_ref()], &program_a.id());
    println!("PDA account: {:?}", pda_account);
    let signature = program_a
        .request()
        .accounts(program_a::accounts::Initialize {
            pda_account,
            signer: alice.pubkey(),
            system_program: system_program::ID,
            // program_b: program_b::ID,
        })
        .args(program_a::instruction::Initialize {})
        .signer(&alice)
        .send()
        .await?;

    println!("Tx sig: {:?}", signature);

    Ok(())
}
