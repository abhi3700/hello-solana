//! Using "Program A" program to test CPI
//!
//! NOTE: Make sure the program is deployed/available on the network being used.
//!
//! Usages:
//! - Transfer SOL from Alice's PDA to Alice.
//! -
//!
//! TODO: Write code to execute concurrently. First 2.

use anchor_client::{
    anchor_lang::system_program, solana_sdk::signature::read_keypair_file, Client, Cluster,
};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Wallet and cluster params
    let admin = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
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
    let client = Client::new_with_options(
        cluster.clone(),
        alice.clone(),
        CommitmentConfig::processed(),
    );

    // Program A instance from ID & Code.
    let program_a = client
        .program(program_a::ID)
        .expect("Program A doesn't exist");

    let (alice_pda_address, _bump) =
        Pubkey::find_program_address(&[b"abhi", alice.pubkey().as_ref()], &program_a.id());

    // Airdrop 100 SOL to alice's PDA.
    // if (bal_of_pda < 10 SOL)
    let admin = Arc::new(admin);
    let rpc_client = solana_rpc_client::rpc_client::RpcClient::new("http://localhost:8899");
    let pda_sol_balance = rpc_client.get_balance(&alice_pda_address)?;

    if pda_sol_balance < 10_000_000_000 {
        let amount = 100_000_000_000; // 100 SOL
        let airdrop_instruction =
            system_instruction::transfer(&admin.pubkey(), &alice_pda_address, amount);
        let recent_blockhash = rpc_client.get_latest_blockhash()?;
        let airdrop_tx = Transaction::new_signed_with_payer(
            &[airdrop_instruction],
            Some(&admin.pubkey()),
            &[&admin],
            recent_blockhash,
        );
        rpc_client.send_and_confirm_transaction(&airdrop_tx)?;
    }

    // Transfer fund from Alice's PDA to Alice using its seed.
    println!("PDA account: {:?}", &alice_pda_address);
    let signature = program_a
        .request()
        .accounts(program_a::accounts::Initialize {
            pda_account: alice_pda_address,
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
