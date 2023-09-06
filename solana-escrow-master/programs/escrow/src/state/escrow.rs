use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Escrow{
    pub maker: Pubkey,
    pub maker_token: Pubkey,
    pub taker_token: Pubkey,
    pub offer_ammount: u64,
    pub seed: u64,
    pub auth_bump: u8,
    pub vault_bumps: u8,
    pub escrow: u8,
}

impl Escrow {
    pub const LEN: usize = (PUBKEY_L *3 ) + (U64_L *2 ) + (U8_L *3);
}