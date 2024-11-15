use anchor_lang::prelude::*;
use program_b::program::ProgramB;

declare_id!("8aKM4FErjs1NUjM4Z8ZyPAcX9b8z52Jw3NFpshyvQXb1");

#[program]
pub mod program_a {
    use super::*;
    use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from Program A");

        let pda_address = ctx.accounts.pda_account.key();
        let signer_address = ctx.accounts.signer.key();
        let bump = ctx.bumps.pda_account;

        // CPI to let system program account to modify lamports of from (PDA) & to (signer) addresses.
        let instruction =
            &system_instruction::transfer(&pda_address, &signer_address, 1_000_000_000);

        // parse infos of all the accounts involved: sender, receiver, system program (for modifying the lamports)
        let account_infos = &[
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.pda_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];
        let signers_seeds: &[&[&[u8]]] = &[&[b"abhi", signer_address.as_ref(), &[bump]]];

        invoke_signed(instruction, account_infos, signers_seeds)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: No need.
    #[account(
        mut,
        seeds = [b"abhi", signer.key().as_ref()],
        bump
    )]
    pub pda_account: AccountInfo<'info>, // or  UncheckedAccount<'info>
    // `mut` used bcoz the intent here is to update the signer's lamports by system program via transferring from signer to PDA.
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    // pub program_b: Program<'info, ProgramB>,
}
