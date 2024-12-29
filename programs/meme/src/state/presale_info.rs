use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct PresaleInfo {
    // Mint address of the presale token
    pub token_mint_address: Pubkey,
    // Softcap amounts in SOL for the presale
    pub hardcap_amount: u64,
    // Total amount of presale tokens available in the presale
    pub deposit_token_amount: u64,
    // Total amount of presale tokens sold during the presale
    // Default is 50 % of the total supply
    pub sold_token_amount: u64,
    // Start time of presale
    pub start_time: u64,
    // End time of presale
    pub end_time: u64,
    // Maximum amount of presale tokens an address can purchase in SOL
    pub max_token_amount_per_address: u64,
    // Min amount of presale tokens an address can purchase in SOL
    pub min_token_amount_per_address: u64,
    // Quote token per presale token
    pub price_per_token: u64,
    // Presale is buyable
    pub is_live: bool,
    // Authority of the presale
    pub authority: Pubkey,
    // Status of hardcapped
    pub is_hard_capped: bool,
}