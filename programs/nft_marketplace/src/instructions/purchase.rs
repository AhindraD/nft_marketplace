use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: Signer<'info>,

    pub maker_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds=[
            b"marketplace",
            marketplace.name.as_str().as_bytes(),
        ],
        bump=marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=maker_mint,
        associated_token::authority=taker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=maker_mint,
        associated_token::authority=listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[
            marketplace.key().as_ref(),
            maker_mint.key().as_ref(),
        ],
        bump=listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
