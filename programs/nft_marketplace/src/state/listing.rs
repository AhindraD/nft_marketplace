use anchor_lang::prelude::*;

use crate::ANCHOR_DISCRIMINATOR;

#[account]
pub struct Listing {
    pub maker: Pubkey,
    pub amount: u64,
    pub mint: Pubkey,
    pub bump: u8,
}

impl Space for Listing {
    const INIT_SPACE: usize = (ANCHOR_DISCRIMINATOR as usize) + 32 + 8 + 32 + 1;
}
