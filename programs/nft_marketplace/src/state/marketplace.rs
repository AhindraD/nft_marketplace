use anchor_lang::prelude::*;

use crate::ANCHOR_DISCRIMINATOR;

#[account]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u16,

    pub bump: u8,
    pub treasury_bump: u8,
    pub rewards_mint_bump: u8,

    pub name: String,
}

impl Space for Marketplace {
    const INIT_SPACE: usize = (ANCHOR_DISCRIMINATOR as usize) + 32 + 2 + 1 + 1 + 1 + (4 + 32);
}
