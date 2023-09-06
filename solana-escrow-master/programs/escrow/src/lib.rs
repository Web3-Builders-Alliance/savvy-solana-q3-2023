use anchor_lang::prelude::*;

mod state;
mod constants;
mod contexts;

use crate::contexts::*;

declare_id!("CzSvoiPzRjUAANWHz824Hw4hPY5PqDHANJai6LDUYubW");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, offer_amount: u64, vault_deposit_amt: u64, seed: u64) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, offer_amount, seed)?;
        ctx.accounts.transfer_to_vault(vault_deposit_amt)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit_to_maker()?;
        ctx.accounts.empty_vault_to_taker()?;
        ctx.accounts.close_vault()
    }

    pub fn update(ctx: Context<Update>, offer_amount: u64) -> Result<()> {
        ctx.accounts.update(offer_amount)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.empty_vault_to_maker()?;
        ctx.accounts.close_vault()
    }
}
