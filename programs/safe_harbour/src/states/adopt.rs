use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Adopt {
    pub agreement: Pubkey,
}
