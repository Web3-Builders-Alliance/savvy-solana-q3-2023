use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Transfer as SplTransfer, transfer as spl_transfer, TokenAccount, Token, Mint};

declare_id!("3R6d7GRQFn1xiDKJDh3JyNMf5TEAPLgcCfc9C4xX6Rt8");

#[program]
pub mod wba_vault {


    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.auth_bump = *ctx.bumps.get("auth").unwrap();
        ctx.accounts.state.vault_bump = *ctx.bumps.get("vault").unwrap();
        ctx.accounts.state.state_bump = *ctx.bumps.get("state").unwrap();
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()>{
        let accounts: Transfer<'_> = Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.vault.to_account_info()
        };

        let cpi: CpiContext<'_, '_, '_, '_, _> = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            accounts
        );

        transfer(cpi, amount)
    }
    
    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()>{
        let accounts: Transfer<'_> = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.owner.to_account_info()
        };

        let seeds: &[&[u8]; 3] = &[
            b"vault",
            ctx.accounts.state.to_account_info().key.as_ref(),
            &[ctx.accounts.state.vault_bump]
        ];

        let pda_signer: &[&[&[u8]]; 1] = &[&seeds[..]];

        let cpi: CpiContext<'_, '_, '_, '_, _> = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            accounts,
            pda_signer
        );

        transfer(cpi, amount)
    }

    pub fn spl_deposit(ctx: Context<SPLDeposit>, amount: u64) -> Result<()> {
        let accounts: SplTransfer<'_> = SplTransfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.owner_ata.to_account_info(),
            authority: ctx.accounts.auth.to_account_info()
        };

        let cpi: CpiContext<'_, '_, '_, '_, _> = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            accounts
        );

        spl_transfer(cpi, amount)
    }

    pub fn spl_withdraw(ctx: Context<SPLDeposit>, amount: u64) -> Result<()> {
        let accounts: SplTransfer<'_> = SplTransfer {
            from: ctx.accounts.owner_ata.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.owner.to_account_info()
        };

        let seeds: &[&[u8]; 3] = &[
            b"auth",
            ctx.accounts.state.to_account_info().key.as_ref(),
            &[ctx.accounts.state.auth_bump]
        ];

        let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];

        let cpi: CpiContext<'_, '_, '_, '_, _> = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            accounts,
            signer_seeds
        );

        spl_transfer(cpi, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        seeds=[b"auth", state.key().as_ref()],
        bump
    )]
    /// Check: This is safe
    auth: UncheckedAccount<'info>,
        #[account(
        seeds=[b"vault", state.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,
    #[account(
        init,
        payer = owner,
        space = VaultState::LEN,
        seeds=[b"state", owner.key().as_ref()],
        bump
    )]
    state: Account<'info, VaultState>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        seeds=[b"vault", state.key().as_ref()],
        bump = state.vault_bump
    )]
    vault: SystemAccount<'info>,
    #[account(
        seeds=[b"state", owner.key().as_ref()],
        bump = state.state_bump
    )]
    state: Account<'info, VaultState>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SPLDeposit<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    owner_ata: Account<'info, TokenAccount>,
    mint: Account<'info, Mint>,
    #[account(
        seeds=[b"auth", state.key().as_ref()],
        bump = state.auth_bump
    )]
    /// Check: This is safe
    auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"spl_vault", state.key().as_ref()],
        token::mint = mint,
        token::authority = auth,
        bump
    )]
    vault: Account<'info, TokenAccount>,
    #[account(
        seeds=[b"state", owner.key().as_ref()],
        bump = state.state_bump
    )]
    state: Account<'info, VaultState>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SPLWithdraw<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    owner_ata: Account<'info, TokenAccount>,
    mint: Account<'info, Mint>,
    #[account(
        seeds=[b"auth", state.key().as_ref()],
        bump = state.auth_bump
    )]
    /// Check: This is safe
    auth: UncheckedAccount<'info>,
    #[account(
        mut,
        close = owner,
        seeds = [b"spl_vault", state.key().as_ref()],
        token::mint = mint,
        token::authority = auth,
        bump
    )]
    vault: Account<'info, TokenAccount>,
    #[account(
        seeds=[b"state", owner.key().as_ref()],
        bump = state.state_bump
    )]
    state: Account<'info, VaultState>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>
}


#[account]
pub struct VaultState {
    auth_bump: u8,
    vault_bump: u8,
    state_bump: u8
}

impl VaultState {
    const LEN: usize = 8 + 3 * 1;
}