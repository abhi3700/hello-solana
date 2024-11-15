use anchor_lang::prelude::*;
use program_b::program::ProgramB;

declare_id!("8aKM4FErjs1NUjM4Z8ZyPAcX9b8z52Jw3NFpshyvQXb1");

#[program]
pub mod program_a {
    use super::*;
    use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

    pub const SEED: &[u8; 4] = b"abhi";

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
        let signer_seeds: &[&[&[u8]]] = &[&[SEED, signer_address.as_ref(), &[bump]]];

        invoke_signed(instruction, account_infos, signer_seeds)?;

        /*
        Call program_b via program_a's one of the PDAs (owned by signer). PDA doesn't have private key. So, it is delegately signed by Alice.
        E.g. Alice calls (using its PDA seed) program-a to call program-b on behalf of Alice's PDA.
        Alice --seed + pk--> [Program-A  [PDA] ...] --> Program-B
        NOTE: Program-A can have so many PDAs so out of all, Alice uses its PDA account (owned by Program-A) to call program-B
        */
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.program_b.to_account_info(),
            program_b::cpi::accounts::Initialize {
                pda_account: ctx.accounts.pda_account.to_account_info(),
            },
            signer_seeds,
        );

        program_b::cpi::initialize(cpi_context)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: No need.
    #[account(
        mut,
        seeds = [program_a::SEED, signer.key().as_ref()],
        bump
    )]
    pub pda_account: AccountInfo<'info>, // or  UncheckedAccount<'info>
    // `mut` used bcoz the intent here is to update the signer's lamports by system program via transferring from signer to PDA.
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub program_b: Program<'info, ProgramB>,
}
