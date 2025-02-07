use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        seeds=[b"marketplace",marketplace.name.as_str().as_bytes()],
        bump=marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub maker_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint=maker_mint,
        associated_token::authority=maker
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer=maker,
        associated_token::mint=maker_mint,
        associated_token::authority=listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer=maker,
        space=Listing::INIT_SPACE,
        seeds=[
            marketplace.key().as_ref(),
            maker_mint.key().as_ref(),
        ],
        bump
    )]
    pub listing: Account<'info, Listing>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
