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
        close=maker,
        seeds=[
            marketplace.key().as_ref(),
            maker_mint.key().as_ref(),
        ],
        bump=listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        mut,
        seeds=[
            b"treasury",
            marketplace.key().as_ref(),
        ],
        bump=marketplace.treasury_bump,
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        seeds=[
            b"rewards",
            marketplace.key().as_ref()
        ],
        bump=marketplace.rewards_mint_bump,
        mint::authority=marketplace,
        mint::decimals=6,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {
    //1. taker send sol to maker
    //2. transfer nft from vault to taker_ata
    //3. close accouts
}
