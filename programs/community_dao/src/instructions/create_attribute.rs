use anchor_lang::prelude::*;

use crate::state::*;

pub fn create_attribute(ctx: Context<CreateAttribute>) -> Result<()> {
    ctx.accounts.sbt.set_inner(Attribute::new(
        data,
        *ctx.bumps.get("attribute").expect("Bump not found"),
    ));

    Ok(())
}

#[derive(Accounts)]
pub struct CreateAttribute<'info> {
    #[account(
        init,
        payer = auth,
        space = size_of::<Attribute>(),
        seeds = [b"attribute", auth.key().as_ref()],
        bump
    )]
    pub attribute: Account<'info, Attribute>,
    #[account(mut)]
    pub auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}