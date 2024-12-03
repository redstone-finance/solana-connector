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
