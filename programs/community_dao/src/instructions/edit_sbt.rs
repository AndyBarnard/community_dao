
use anchor_lang::prelude::*;

use crate::state::{Sbt};

pub fn edit_sbt<T: Clone, const N: usize>(
    ctx: Context<EditSbt<T, N>>,
    new_sbt_data: [T; N],
) -> Result<()> {
    ctx.accounts.sbt.data = new_sbt_data;

    Ok(())
}

//TODO: add has_one for validation  I think on all these PDAs
#[derive(Accounts)]
pub struct EditSbt<'info, T: Clone, const N: usize> {
    #[account(
        mut,
        seeds = [b"sbt", auth.key().as_ref()],
        bump = sbt.bump,
    )]
    pub sbt: Account<'info, Sbt<T, N>>,
    pub auth: Signer<'info>,
}
