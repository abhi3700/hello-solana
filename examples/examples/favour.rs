//! Using "Favour" program
//!
//! NOTE: Make sure the program is deployed/available on the network being used.
//!
//! - Alice set/update its favourites.
//! - Get Alice's favourites.
//!
//! TODO: Write code to execute concurrently. First 2.

use anchor_client::{
    anchor_lang::system_program, solana_sdk::signature::read_keypair_file, Client, Cluster,
};
use favour::Favourites;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Wallet and cluster params.
    let alice = read_keypair_file(&*shellexpand::tilde("~/.config/solana/alice.json"))
        .expect("Example requires a keypair file");
    // For local testnet. Comment/uncomment.
    let url = Cluster::Custom(
        "http://localhost:8899".to_string(),
        "ws://127.0.0.1:8900".to_string(),
    );

    // let url = Cluster::Devnet;

    // Client.
    let alice = Arc::new(alice);
    let client =
        Client::new_with_options(url.clone(), alice.clone(), CommitmentConfig::processed());

    // get instance
    let favour = client.program(favour::ID).expect("Program doesn't exist");

    // create a PDA acount to store Alice's data
    let seed = b"favourites";
    let (alice_fav_pda, _bump) =
        Pubkey::find_program_address(&[seed, alice.pubkey().as_ref()], &favour::ID);

    // Alice sets its favourites.
    let signature = favour
        .request()
        .accounts(favour::accounts::SetFavourites {
            favourites: alice_fav_pda,
            user: alice.pubkey(),
            system_program: system_program::ID,
        })
        .args(favour::instruction::SetFavourites {
            favourites: Favourites {
                num: 3700,
                color: "red".to_string(),
                hobbies: vec!["biking".to_string(), "music".to_string()],
            },
        })
        .signer(&alice)
        .send()
        .await?;
    println!("Alice set favourites. Tx signature: {:?}", signature);

    let alice_favourites: Favourites = favour.account(alice_fav_pda).await?;
    println!("Alice's favourites: {:#?}", alice_favourites);

    // ======= Bob
    let bob = read_keypair_file(&*shellexpand::tilde("~/.config/solana/bob.json"))
        .expect("Requires bob's keypair file");
    let bob = Arc::new(bob);

    let (bob_fav_pda, _bump) =
        Pubkey::find_program_address(&[seed, bob.pubkey().as_ref()], &favour::ID);
    let bob_favourites = Favourites {
        num: 7,
        color: "pink".to_string(),
        hobbies: vec!["football".to_string(), "eating".to_string()],
    };
    let signature = favour
        .request()
        .accounts(favour::accounts::SetFavourites {
            favourites: bob_fav_pda,
            user: bob.pubkey(),
            system_program: system_program::ID,
        })
        .args(favour::instruction::SetFavourites {
            favourites: bob_favourites,
        })
        .signer(&bob)
        .send()
        .await?;
    println!("Bob set favourites. Tx signature: {:?}", signature);
    let bob_favourites: Favourites = favour.account(bob_fav_pda).await?;
    println!("Bob's favourites: {:#?}", bob_favourites);

    // Get error if Bob tries to set favourites for Alice.
    let err = favour
        .request()
        .accounts(favour::accounts::SetFavourites {
            favourites: alice_fav_pda,
            user: bob.pubkey(),
            system_program: system_program::ID,
        })
        .args(favour::instruction::SetFavourites {
            favourites: bob_favourites,
        })
        .signer(&bob)
        .send()
        .await
        .err()
        .unwrap();
    println!("Bob set favourites for Alice. Error: {:?}", err);

    Ok(())
}
