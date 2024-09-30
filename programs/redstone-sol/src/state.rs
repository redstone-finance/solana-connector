use anchor_lang::prelude::*;

pub type SignerAddress = [u8; 20];
pub type FeedId = [u8; 32];

#[account]
#[derive(Default)]
pub struct PriceData {
    pub feed_id: FeedId,
    pub value: u128,
    pub timestamp: u64,
}
