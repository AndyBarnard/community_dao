use anchor_lang::prelude::*;

#[account]
pub struct Attribute {
    //this field should be able to hold basically any data type, not sure if it can be
    //done in the struct like this or if we have to serialize the data first and
    //have this field be byte type
    data: String,  
    bump: u8, 
}

// each Attribute can be like 8 bytes or some standard size that can hold enough data

impl Attribute {
    pub fn new(data: String, bump: u8) -> Self {
        Attribute {
            data,
            bump,
        }
    }
}