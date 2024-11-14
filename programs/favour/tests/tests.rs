//! FIXME: Resolve `processor` use related error, which stems from the incompatibility
//! of using Associated SPL token lib, it's version is incompatible with that of Anchor.
//! Currently the latest version of solana-sdk is not yet bumped to, by Anchor.
//! That's why using older version of solana-sdk. In general, all the packages by Solana Labs.

#[cfg(test)]
mod tests {
    use {
        anchor_lang::{InstructionData, ToAccountMetas},
        favour::Favourites,
        solana_program_test::*,
        solana_sdk::{
            instruction::Instruction,
            pubkey::Pubkey,
            signature::{Keypair, Signer},
            transaction::Transaction,
        },
        spl_associated_token_account::processor,
    };

    #[tokio::test]
    async fn set_favourites_is_ok() {
        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new(
            "favour",
            program_id,
            processor!(processor::process_instruction),
        );

        // Start the test environment
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create a user keypair
        let alice = Keypair::new();
        let bob = Keypair::new();

        // airdrop SOL to the users
        let airdrop_amount = 1_000_000_000; // 1 SOL
        let airdrop_instruction = solana_sdk::system_instruction::transfer(
            &payer.pubkey(),
            &alice.pubkey(),
            airdrop_amount,
        );
        let airdrop_tx = Transaction::new_signed_with_payer(
            &[airdrop_instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );
        banks_client.process_transaction(airdrop_tx).await.unwrap();

        let seed = b"favourites";
        let (alice_fav_pda, _bump) =
            Pubkey::find_program_address(&[seed, alice.pubkey().as_ref()], &favour::ID);

        let instruction = Instruction {
            program_id,
            accounts: favour::accounts::SetFavourites {
                favourites: alice_fav_pda,
                user: alice.pubkey(),
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None),
            data: favour::instruction::SetFavourites {
                num: 3700,
                color: "blue".to_string(),
                hobbies: vec!["biking".to_string(), "music".to_string()],
            }
            .data(),
        };

        // Create and process the transaction
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&alice.pubkey()),
            &[&alice],
            recent_blockhash,
        );

        banks_client.process_transaction(tx).await.unwrap();

        // Fetch the alice's PDA data
        let alice_pda_account = banks_client
            .get_account(alice_fav_pda)
            .await
            .unwrap()
            .unwrap();

        // Deserialize the account
        let alice_favourites: Favourites =
            anchor_lang::AccountDeserialize::try_deserialize(&mut alice_pda_account.data.as_ref())
                .unwrap();

        println!("{:?}", alice_favourites);
    }
}
