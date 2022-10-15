use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};

#[account]
#[derive(BorshSerialize, BorshDeserialize)]
// #[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Sbt<T, const N: usize> {
    pub name: String,
    pub data: [T; N],
    pub bump: u8,
}

impl<T, const N: usize> Sbt<T, N> {
    pub fn new(name: String, data: [T; N], bump: u8) -> Self {
        Sbt {
            name,
            data,
            bump,
        }
        //now take `data` and create an Attribute for each field
    }
}