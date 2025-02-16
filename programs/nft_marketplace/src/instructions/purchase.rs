use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Listing, Marketplace};

//HOMEWORK
//Reward the maker and taker for participating in marketplace
//use reward token mint as Reward
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
    //1. taker send sol to maker & maker sends fee to treasury
    pub fn pay(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts_options1 = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
        };
        let cpi_ctx1 = CpiContext::new(cpi_program, cpi_accounts_options1);
        let nft_price = self.listing.price;
        //taker sending full price to maker
        transfer(cpi_ctx1, nft_price)?;

        let cpi_program = self.system_program.to_account_info();
        let cpi_account_options2 = Transfer {
            from: self.maker.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx2 = CpiContext::new(cpi_program, cpi_account_options2);
        let marketplace_fee = self.marketplace.fee as u64;
        //maker paying the fee from sale made
        transfer(cpi_ctx2, marketplace_fee)?;
        Ok(())
    }
    //2. transfer nft from vault to taker_ata
    //3. close accouts
}
