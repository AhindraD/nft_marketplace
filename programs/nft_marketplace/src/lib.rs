pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("72kseoyebwFZ2yXzVm2CTgcEgqVyxinxumhYYuA8Rind");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.pay()?;
        ctx.accounts.transfer_nft()?;
        ctx.accounts.close_vault()?;
        Ok(())
    }
}
