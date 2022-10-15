use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Soul {
    pub name: String,
    pub bump: u8,
}

impl Soul {
    pub fn new(name: String, bump: u8) -> Self {
        Soul {
            name,
            bump,
        }
    }
}