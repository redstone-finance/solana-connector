pub fn u256_to_string(u256: &[u8; 32]) -> String {
    u256.iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c as char)
        .collect()
}

pub fn u256_to_num_string(u256: &[u8; 32]) -> String {
    let mut num = 0u128;
    for &c in u256 {
        num = (num << 8) | c as u128;
    }
    num.to_string()
}

#[cfg(feature = "dev")]
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    bytes.iter().fold(
        String::with_capacity(bytes.len() * 2),
        |mut output, b| {
            let _ = write!(output, "{:02x}", b);
            output
        },
    )
}
