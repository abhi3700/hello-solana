use anchor_lang::prelude::*;

declare_id!("3dD57WoTbyuj716SqX5FsX8mQ5C4U9sUoXxEq8Sp2KLv");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, value: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.counter.count += 1;
        msg!("Incremented counter to ");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init , payer = user, space = 8 + Counter::INIT_SPACE)]
    counter: Account<'info, Counter>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    counter: Account<'info, Counter>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    count: u64,
}
