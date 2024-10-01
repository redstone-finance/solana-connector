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

pub fn calculate_median(values: &mut [U256]) -> U256 {
    if values.is_empty() {
        return [0u8; 32];
    }

    values.sort_unstable();
    let len = values.len();

    if len % 2 == 0 {
        let mid = len / 2;
        let left = &values[mid - 1];
        let right = &values[mid];
        let sum = add_u256(left, right);
        divide_u256_by_2(&sum)
    } else {
        values[len / 2]
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

    result
}

fn divide_u256_by_2(a: &U256) -> U256 {
    let mut result = [0u8; 32];
    let mut carry = 0u8;

    for i in 0..32 {
        let current = (carry << 7) | (a[i] >> 1);
        result[i] = current;
        carry = a[i] & 1;
    }

    result
}
