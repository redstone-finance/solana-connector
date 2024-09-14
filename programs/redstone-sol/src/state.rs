use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct DataPoint {
    pub feed_id: [u8; 32],
    pub value: u128,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct DataPackage {
    pub signer_address: Vec<u8>,
    pub timestamp: u64,
    pub data_points: Vec<DataPoint>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Payload {
    pub data_packages: Vec<DataPackage>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Config {
    pub signer_count_threshold: u8,
    pub block_timestamp: u64,
    pub signers: Vec<Vec<u8>>,
    pub feed_ids: Vec<[u8; 32]>,
}

#[derive(Accounts)]
pub struct ProcessPayload<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
