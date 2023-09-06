use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{TokenAccount, Mint, Token, Transfer, transfer}
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = maker_token,
        associated_token::authority = maker,
    )]
    pub maker_ata: Account<'info, TokenAccount>,

    pub maker_token: Box<Account<'info, Mint>>,
    pub taker_token: Box<Account<'info, Mint>>,

    #[account(
        seeds = [b"auth", escrow.key().as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because this account doen't exist 
    pub auth: UncheckedAccount<'info>,

    #[account(
        init,
        payer = maker,
        seeds = [b"vault", escrow.key().as_ref()],
        bump,
        token::mint = maker_token,
        token::authority = auth,
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = Escrow::LEN,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Box<Account<'info, Escrow>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_account: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

impl<'info> Make<'info>  {
    pub fn init(&mut self, bumps: &BTreeMap<String, u8>, offer_amount: u64, seed: u64) -> Result<()> {
        let escrow = &mut self.escrow;
        escrow.maker = *self.maker.key;
        escrow.maker_token = self.maker_token.key();
        escrow.taker_token = self.taker_token.key();
        escrow.offer_amount = offer_amount;
        escrow.seed = seed;
        escrow.auth_bump = *bumps.get("auth").unwrap();
        escrow.vault_bump = *bumps.get("vault").unwrap();
        escrow.escrow_bump = *bumps.get("escrow").unwrap();

        Ok(())
    }

    pub fn transfer_to_vault(&self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.maker_ata.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer(cpi_ctx, amount)
    }
}