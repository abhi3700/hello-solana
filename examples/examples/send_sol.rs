//! Transfer SOL
//!
//! ## Usage
//! - Alice sends SOL to Bob
//! - Alice sends SOL to [Bob, Charlie, David, Eve, Frank]

use {
    solana_sdk::{
        pubkey::Pubkey,
        signer::{keypair::read_keypair_file, Signer},
        transaction::Transaction,
    },
    std::{str::FromStr, sync::Arc},
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Wallet and cluster params.
    let alice = read_keypair_file(&*shellexpand::tilde("~/.config/solana/alice.json"))
        .expect("Example requires a keypair file");

    // users
    let alice = Arc::new(alice);
    let bob = Pubkey::from_str("7i35XMGWhbrKBsQemMRVeAHCwxNKpnWyNZB7d9NMsQs1")?;
    let charlie = Pubkey::from_str("J3v5ERfLWFFax2fLHZALo3k24B9SRp4NS8QwyEzvjwGD")?;
    let david = Pubkey::from_str("7pbadVrGBdH3hheFt6444h7HPkofH15dxcjRy7mLySWr")?;
    let eve = Pubkey::from_str("J9UWJFC9ViRqF4JUnWcUzCy8WsJLn5ZTxButzXNabhsR")?;
    let frank = Pubkey::from_str("DdSX6JDnN4KmBbc5pSDW7e18uT43R2MiWWwvE268wSJc")?;

    // Client.
    let client = solana_rpc_client::rpc_client::RpcClient::new("http://localhost:8899");
    // let client = solana_rpc_client::rpc_client::RpcClient::new(Cluster::Devnet.url());
    let amount = 1_000_000_000;

    // Create an instruction set. Here only 1 though.
    let transfer_sol_instr =
        solana_sdk::system_instruction::transfer(&alice.pubkey(), &bob, amount);
    let latest_blockhash = client.get_latest_blockhash()?;
    let tx1 = Transaction::new_signed_with_payer(
        &[transfer_sol_instr],
        Some(&alice.pubkey()),
        &[&alice],
        latest_blockhash,
    );
    // TODO: Need to look for a async one.
    let tx_hash = client.send_and_confirm_transaction(&tx1)?;
    println!("Transfer to 1. Signature: {:?}", tx_hash);

    // -- To many
    let transfer_sol_many_instr = solana_sdk::system_instruction::transfer_many(
        &alice.pubkey(),
        &[
            (bob, amount),
            (charlie, amount),
            (david, amount),
            (eve, amount),
            (frank, amount),
        ],
    );
    let latest_blockhash = client.get_latest_blockhash()?;
    let tx2 = Transaction::new_signed_with_payer(
        &transfer_sol_many_instr,
        Some(&alice.pubkey()),
        &[&alice],
        latest_blockhash,
    );
    let tx_hash = client.send_and_confirm_transaction(&tx2)?;
    println!("Transfer to many. Signature: {:?}", tx_hash);

    // M-2

    Ok(())
}
