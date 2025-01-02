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

/// Log a msg in dev mode
pub fn debug_msg<F: Fn() -> String>(_msg_fn: F) {
    #[cfg(feature = "dev")]
    {
        use anchor_lang::prelude::msg;
        msg!("{}", _msg_fn())
    }
}
