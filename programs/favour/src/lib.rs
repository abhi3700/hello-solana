use anchor_lang::prelude::*;

declare_id!("89Cn2GkJEBaSEMmMuz7ftyDq5EWKR1kPEHCW56wSYeeV");

#[program]
pub mod favour {
    use super::*;

    pub fn set_favourites(ctx: Context<SetFavourites>, favourites: Favourites) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        // M-1
        // ctx.accounts.favourites.num = favourites.num;
        // ctx.accounts.favourites.color = favourites.color;
        // ctx.accounts.favourites.hobbies = favourites.hobbies;

        // M-2
        ctx.accounts.favourites.set_inner(favourites);

        let user_pk = ctx.accounts.user.key();
        msg!(&format!("User {user_pk}"));
        Ok(())
    }
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Favourites {
    pub num: u64,
    #[max_len(50)]
    pub color: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavourites<'info> {
    #[account(
        init_if_needed, 
        payer = user, 
        space = 8 + Favourites::INIT_SPACE,
        // fav...+user's pk as seed for each user.
        seeds = [b"favourites", user.key().as_ref()],
        // to calculate the seeds.
        bump

    )]
    pub favourites: Account<'info, Favourites>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
