use anchor_lang::prelude::*;

use crate::util::U256;

pub type SignerAddress = [u8; 20];
pub type FeedId = U256;

#[account]
#[derive(Default)]
pub struct PriceData {
    pub feed_id: FeedId,
    pub value: U256,
    pub timestamp: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct DataPoint {
    pub feed_id: FeedId,
    pub value: U256,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct DataPackage {
    pub signer_address: SignerAddress,
    pub timestamp: u64,
    pub data_points: Vec<DataPoint>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Payload {
    pub data_packages: Vec<DataPackage>,
}

pub struct Config {
    pub signer_count_threshold: u8,
    pub block_timestamp: u64,
    pub signers: [SignerAddress; 10],
}
