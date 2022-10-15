use anchor_lang::prelude::*;
// use borsh::{BorshSerialize, BorshDeserialize};

use instructions::*;
// use state::*;

pub mod instructions;
pub mod state;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sbts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_soul(ctx: Context<CreateSoul>, name: String) -> Result<()> {
        instructions::create_soul(ctx, name)
    }

    pub fn create_sbt<T: Clone, const N: usize>(
        ctx: Context<CreateSbt<T, N>>,
        name: String,
        data: String,
        attributes: Vec<u8>,
    ) -> Result<()> {
        instructions::create_sbt(ctx, name, data, attributes)
    }

    pub fn edit_sbt<T: Clone, const N: usize>(
        ctx: Context<EditSbt<T, N>>,
        new_sbt_data: String,
    ) -> Result<()> {
        instructions::edit_sbt(ctx, new_sbt_data)
    }
}


#[derive(Accounts)]
pub struct Initialize {}
