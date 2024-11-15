use anchor_lang::prelude::*;

declare_id!("C8opAHa35tZSdbZ4D852qhoK14y2tgNGttVxv8L9adeL");

#[program]
pub mod program_b {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from program-b");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pda_account: Signer<'info>,
}
