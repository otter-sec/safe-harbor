use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Default)]
pub struct Adopt {
    pub agreement: Pubkey,
}
impl Adopt {
    pub const ADOPT_SEED: &'static [u8] = b"adopt_v2";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adopt_default() {
        let adopt = Adopt::default();
        assert_eq!(adopt.agreement, Pubkey::default());
    }
}
