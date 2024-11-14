use anchor_lang::prelude::*;

declare_id!("AGfatHL8AvqujDFVqWJcoQojPza1bznfrQDGyXB2XUna");

#[program]
pub mod favour {
    use super::*;

    pub fn set_favourites(ctx: Context<SetFavourites>, num: u64, color: String, hobbies: Vec<String>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        // M-1
        // ctx.accounts.favourites.num = num;
        // ctx.accounts.favourites.color = color;
        // ctx.accounts.favourites.hobbies = hobbies;

        // M-2
        ctx.accounts.favourites.set_inner(Favourites { num, color, hobbies });

        let user_pk = ctx.accounts.user.key();
        msg!(&format!("User {user_pk}"));
        Ok(())
    }
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Favourites {
    num: u64,
    #[max_len(50)]
    color: String,
    #[max_len(5, 50)]
    hobbies: Vec<String>,
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
    favourites: Account<'info, Favourites>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>,
}
