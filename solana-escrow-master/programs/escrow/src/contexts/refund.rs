use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer, CloseAccount, close_account}, associated_token::AssociatedToken};


use crate::state::Escrow;


#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = taker_token,
        associated_token::authority = maker
    )]
    pub maker_ata: Account<'info, TokenAccount>,

    pub maker_token: Box<Account<'info, Mint>>,
    pub taker_token: Box<Account<'info, Mint>>,

    #[account(
        seeds = [b"auth", escrow.key().as_ref()],
        bump = escrow.auth_bump,
    )]
    /// CHECK: This is not dangerous because this account doesn't exist
    pub auth: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", escrow.key().as_ref()],
        bump = escrow.vault_bump,
        token::mint = maker_token,
        token::authority = auth
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        has_one = maker,
        has_one = maker_token,
        seeds = [b"vault", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.escrow_bump,
        close = maker
    )]
    pub escrow: Box<Account<'info, Escrow>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

impl<'info> Refund<'info> {
    pub fn empty_vault_to_maker(&self) -> Result<()> {
        let signer_seeds = &[&b"auth"[..], &[self.escrow.auth_bump]];
        let signer_seeds_pda = &[&signer_seeds[..]];

        let cpi_accounts = Transfer {
            from: self.vault_ata.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds_pda
        );

        transfer(cpi_ctx, self.vault_ata.amount)
    }

    pub fn close_vault(&self) -> Result<()> {
        let signer_seeds = &[&b"auth"[..], &[self.escrow.auth_bump]];
        let signer_seeds_pda = &[&signer_seeds[..]];

        let cpi_accounts = CloseAccount {
            account: self.vault_ata.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.auth.to_account_info()
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            cpi_accounts, 
            signer_seeds_pda
        );

        close_account(cpi_ctx)
    }
}