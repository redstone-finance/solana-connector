pub type U256 = [u8; 32];
pub const U256_MAX: U256 = [0xFF; 32];

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

pub fn u256_to_string(u256: U256) -> String {
    u256.iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c as char)
        .collect()
}

pub fn u256_to_num_string(u256: U256) -> String {
    let mut num = 0u128;
    for &c in &u256 {
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

pub trait FromBytesRepr<T: Sized>: Sized {
    fn from_bytes(bytes: T) -> Self;
}

impl FromBytesRepr<&[u8]> for U256 {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut array = [0u8; 32];
        let len = bytes.len().min(32);
        array[..len].copy_from_slice(&bytes[..len]);
        array
    }
}

impl FromBytesRepr<&[u8]> for usize {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.iter().fold(0usize, |acc, &b| (acc << 8) | b as usize)
    }
}

impl FromBytesRepr<&[u8]> for u64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.iter().fold(0u64, |acc, &b| (acc << 8) | b as u64)
    }
}

impl FromBytesRepr<&[u8]> for u128 {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.iter().fold(0u128, |acc, &b| (acc << 8) | b as u128)
    }
}

pub fn calculate_median(values: &mut [U256]) -> Option<U256> {
    if values.is_empty() {
        return None;
    }

    values.sort_unstable();
    let len = values.len();

    if len % 2 == 0 {
        let mid = len / 2;
        let left = &values[mid - 1];
        let right = &values[mid];
        let sum = add_u256(left, right);
        Some(divide_u256_by_2(&sum))
    } else {
        Some(values[len / 2])
    }
}

fn add_u256(a: &U256, b: &U256) -> U256 {
    let mut result = [0u8; 32];
    let mut carry = 0u16;

    for i in (0..32).rev() {
        let sum = u16::from(a[i]) + u16::from(b[i]) + carry;
        result[i] = sum as u8;
        carry = sum >> 8;
    }

    if carry > 0 {
        U256_MAX
    } else {
        result
    }
}

fn divide_u256_by_2(a: &U256) -> U256 {
    if a == &U256_MAX {
        return U256_MAX;
    }

    let mut result = [0u8; 32];
    let mut carry = 0u8;

    for i in 0..32 {
        let current = (carry << 7) | (a[i] >> 1);
        result[i] = current;
        carry = a[i] & 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_with_max_values() {
        let mut values = vec![U256_MAX; 100];
        let result = calculate_median(&mut values);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), U256_MAX);
    }
}
