use anchor_lang::prelude::*;

pub type SignerAddressBs = [u8; 20];
pub type FeedIdBs = [u8; 32];
pub type ValueBs = [u8; 32];

#[account]
#[derive(Default)]
pub struct PriceData {
    pub feed_id: FeedIdBs,
    pub value: ValueBs,
    pub timestamp: u64,
    pub write_timestamp: u64,
}

#[account]
pub struct ConfigAccount {
    pub owner: Pubkey,
    pub signer_count_threshold: u8,
    pub signers: Vec<SignerAddressBs>,
    pub max_timestamp_delay_ms: u64,
    pub max_timestamp_ahead_ms: u64,
}
