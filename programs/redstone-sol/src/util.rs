pub type U256 = [u8; 32];

pub fn u256_to_string(u256: U256) -> String {
    u256.iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c as char)
        .collect()
}

pub fn u256_from_slice(bytes: &[u8]) -> U256 {
    let mut array = [0u8; 32];
    let len = if bytes.len() > 32 { 32 } else { bytes.len() };
    array[..len].copy_from_slice(&bytes[..len]);
    array
}
