use anchor_lang::prelude::*;
use std::mem::*;

use crate::state::Soul;

pub fn create_soul(ctx: Context<CreateSoul>, name: String) -> Result<()> {
    ctx.accounts.soul.set_inner(Soul::new(
        name,
        *ctx.bumps.get("soul").expect("Bump not found"),
    ));

    Ok(())
}

#[derive(Accounts)]
pub struct CreateSoul<'info> {
    #[account(
        init,
        payer = auth,
        space = size_of::<Soul>(),
        seeds = [b"soul", auth.key().as_ref()],
        bump
    )]
    pub soul: Account<'info, Soul>,
    #[account(mut)]
    pub auth: Signer<'info>,
    //TODO: pass as seed? for now we can assume the signer is the User, i.e.
    //user creates their own Soul. later another protocol will be able to 
    //create a soul on behalf of a user
    // pub user: Account<'info>,
    pub system_program: Program<'info, System>,
}
