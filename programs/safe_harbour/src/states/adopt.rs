use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Adopt {
    pub agreement: Pubkey,
}
impl Adopt {
    pub const ADOPT_SEED: &[u8] = b"adopt_v2";
}
