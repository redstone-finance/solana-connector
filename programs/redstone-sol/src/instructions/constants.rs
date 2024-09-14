pub const REDSTONE_MARKER: [u8; 9] = [0, 0, 2, 237, 87, 1, 30, 0, 0];
pub const DATA_FEED_ID_BS: usize = 32;
pub const SIGNATURE_BS: usize = 65;
pub const TIMESTAMP_BS: usize = 6;
pub const DATA_POINTS_COUNT_BS: usize = 3;
pub const DATA_POINT_VALUE_BYTE_SIZE_BS: usize = 4;
pub const MAX_TIMESTAMP_DELAY_MS: u64 = 15 * 60 * 1000; // 15 minutes
pub const MAX_TIMESTAMP_AHEAD_MS: u64 = 3 * 60 * 1000; // 3 minutes
