use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::Marketplace;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer=admin,
        space=Marketplace::INIT_SPACE,
        seeds=[
            b"marketplace",
            name.as_str().as_bytes(),
        ],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        seeds=[
            b"treasury",
            marketplace.key().as_ref()
            ],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        init,
        payer=admin,
        seeds=[
            b"rewards",
            marketplace.key().as_ref()
        ],
        bump,
        mint::authority=admin,
        mint::decimals=9,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, fee: u16, bumps: InitializeBumps) -> Result<()> {
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            rewards_mint_bump: bumps.rewards_mint,
            name,
        });
        Ok(())
    }
}
