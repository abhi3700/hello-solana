pub fn airdrop(
    rpc_client: solana_rpc_client::rpc_client::RpcClient,
    from: &dyn solana_sdk::signer::Signer,
    to_lamports: &[(solana_sdk::pubkey::Pubkey, u64)],
) -> eyre::Result<solana_sdk::signature::Signature> {
    let airdrop_instruction =
        solana_sdk::system_instruction::transfer_many(&from.pubkey(), to_lamports);
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let airdrop_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &airdrop_instruction,
        Some(&from.pubkey()),
        &[from],
        recent_blockhash,
    );
    let tx_sig = rpc_client.send_and_confirm_transaction(&airdrop_tx)?;

    Ok(tx_sig)
}
