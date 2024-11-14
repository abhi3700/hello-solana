use anchor_lang::prelude::*;

declare_id!("D6eF6jXyjA7ZcD3giapWYL8wEP4t4AJCWBa352tMjEZY");

#[program]
pub mod foo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
