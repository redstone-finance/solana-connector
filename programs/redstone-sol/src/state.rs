use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct PriceData {
    pub feed_id: [u8; 32],
    pub value: u128,
    pub timestamp: u64,
}

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
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<PriceData>(),
        seeds = [b"price", b"ETH\0\0"],
        bump
    )]
    pub eth_price_account: Account<'info, PriceData>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<PriceData>(),
        seeds = [b"price", b"BTC\0\0"],
        bump
    )]
    pub btc_price_account: Account<'info, PriceData>,
    pub system_program: Program<'info, System>,
}
