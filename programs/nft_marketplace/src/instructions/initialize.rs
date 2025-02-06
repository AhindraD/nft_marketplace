use anchor_lang::prelude::*;

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

    pub system_program: Program<'info, System>,
}
