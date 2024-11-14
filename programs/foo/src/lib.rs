use anchor_lang::prelude::*;

declare_id!("9zKj7KkRHEjUtzBbF93sCdvJNhDug92959urdnGJHnw4");

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
