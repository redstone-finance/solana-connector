use crate::constants::SIGNERS;
use crate::error::RedstoneError;
use crate::redstone;
use crate::state::*;
use crate::util::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ProcessPayload<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<PriceData>(),
        seeds = [b"price", b"ETH\0\0"],
        bump,
        constraint = eth_price_account.to_account_info().owner == __program_id
    )]
    pub eth_price_account: Account<'info, PriceData>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<PriceData>(),
        seeds = [b"price", b"BTC\0\0"],
        bump,
        constraint = btc_price_account.to_account_info().owner == __program_id
    )]
    pub btc_price_account: Account<'info, PriceData>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<PriceData>(),
        seeds = [b"price", b"AVAX\0"],
        bump,
        constraint = avax_price_account.to_account_info().owner == __program_id
    )]
    pub avax_price_account: Account<'info, PriceData>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<PriceData>(),
        seeds = [b"price", b"USDC\0"],
        bump,
        constraint = usdc_price_account.to_account_info().owner == __program_id
    )]
    pub usdc_price_account: Account<'info, PriceData>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<PriceData>(),
        seeds = [b"price", b"LINK\0"],
        bump,
        constraint = link_price_account.to_account_info().owner == __program_id
    )]
    pub link_price_account: Account<'info, PriceData>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<PriceData>(),
        seeds = [b"price", b"SOL\0\0"],
        bump,
        constraint = sol_price_account.to_account_info().owner == __program_id
    )]
    pub sol_price_account: Account<'info, PriceData>,
    pub system_program: Program<'info, System>,
}

pub fn process_redstone_payload(
    ctx: Context<ProcessPayload>,
    payload: Vec<u8>,
) -> Result<()> {
    // block_timestamp as milis
    let block_timestamp = Clock::get()?.unix_timestamp as u64 * 1000;
    let config = Config {
        block_timestamp,
        signer_count_threshold: 1,
        signers: SIGNERS,
        feed_ids: [
            u256_from_slice("ETH".as_bytes()),
            u256_from_slice("BTC".as_bytes()),
            u256_from_slice("AVAX".as_bytes()),
            u256_from_slice("USDC".as_bytes()),
            u256_from_slice("LINK".as_bytes()),
            u256_from_slice("SOL".as_bytes()),
        ],
    };
    redstone::verify_redstone_marker(&payload)?;

    let mut payload = payload;
    let payload = redstone::parse_raw_payload(&mut payload)?;

    redstone::verify_data_packages(&payload, &config)?;

    #[cfg(feature = "dev")]
    {
        msg!(
            "Payload processed successfully: {}",
            payload.data_packages.len()
        );
        for package in &payload.data_packages {
            msg!(
                "Package signer: 0x{}",
                bytes_to_hex(&package.signer_address)
            );
            for data_point in &package.data_points {
                msg!(
                    "Data point: {} {}",
                    u256_to_string(data_point.feed_id),
                    data_point.value.to_string()
                );
            }
        }
    }

    for package in &payload.data_packages {
        // all of the data points have the same values, take the first one
        // at this stage, all of the signatures are verified
        if let Some(data_point) = &package.data_points.first() {
            let price_account = match data_point.feed_id {
                id if id == u256_from_slice("ETH".as_bytes()) => {
                    &mut ctx.accounts.eth_price_account
                }
                id if id == u256_from_slice("BTC".as_bytes()) => {
                    &mut ctx.accounts.btc_price_account
                }
                id if id == u256_from_slice("AVAX".as_bytes()) => {
                    &mut ctx.accounts.avax_price_account
                }
                id if id == u256_from_slice("USDC".as_bytes()) => {
                    &mut ctx.accounts.usdc_price_account
                }
                id if id == u256_from_slice("LINK".as_bytes()) => {
                    &mut ctx.accounts.link_price_account
                }
                id if id == u256_from_slice("SOL".as_bytes()) => {
                    &mut ctx.accounts.sol_price_account
                }
                _ => return Err(RedstoneError::InvalidPriceAccount.into()),
            };

            price_account.value = data_point.value;
            price_account.timestamp = config.block_timestamp;
            price_account.feed_id = data_point.feed_id;

            msg!(
                "Updated price for feed {}: {} at timestamp {}",
                u256_to_string(data_point.feed_id),
                price_account.value,
                price_account.timestamp
            );
        }
    }

    Ok(())
}
