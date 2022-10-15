use anchor_lang::prelude::*;
use std::mem::*;

use crate::state::{Sbt, Soul};

pub fn create_sbt<T: Clone, const N: usize>(
    ctx: Context<CreateSbt<T, N>>,
    name: String,
    data: [T; N],
    attributes: Vec<u8>, //we iterate over each attribute and derive an Attribute PDA for each on this SBT
) -> Result<()> {
    ctx.accounts.sbt.set_inner(Sbt::new(
        name,
        data,
        *ctx.bumps.get("soul").expect("Bump not found"),
    ));

    Ok(())
}

#[derive(Accounts)]
pub struct CreateSbt<'info, T: Clone, const N: usize> {
    #[account(
        init,
        payer = auth,
        space = size_of::<Sbt<T, N>>(),
        seeds = [b"sbt", auth.key().as_ref()],
        bump
    )]
    pub sbt: Account<'info, Sbt<T, N>>,
    #[account(mut)]
    pub auth: Signer<'info>,
    pub soul: Account<'info, Soul>, //
    pub system_program: Program<'info, System>,
}
