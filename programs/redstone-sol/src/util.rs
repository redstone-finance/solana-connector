pub type U256 = [u8; 32];

pub trait Trim<T>
where
    Self: Sized,
{
    fn trim_end(&mut self, len: usize) -> T;
}

impl Trim<Vec<u8>> for Vec<u8> {
    fn trim_end(&mut self, len: usize) -> Self {
        if len >= self.len() {
            std::mem::take(self)
        } else {
            self.split_off(self.len() - len)
        }
    }
}

// helper, debug, can be deleted later
pub fn u256_to_string(u256: U256) -> String {
    u256.iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c as char)
        .collect()
}

// helper, debug, can be deleted later
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

pub fn u256_from_slice(bytes: &[u8]) -> U256 {
    let mut array = [0u8; 32];
    let len = if bytes.len() > 32 { 32 } else { bytes.len() };
    array[..len].copy_from_slice(&bytes[..len]);
    array
}

pub fn vec_to_usize(vec: &[u8]) -> usize {
    vec.iter().fold(0usize, |acc, &b| (acc << 8) | b as usize)
}

pub fn vec_to_u64(vec: &[u8]) -> u64 {
    vec.iter().fold(0u64, |acc, &b| (acc << 8) | b as u64)
}

pub fn vec_to_u128(vec: &[u8]) -> u128 {
    vec.iter().fold(0u128, |acc, &b| (acc << 8) | b as u128)
}
