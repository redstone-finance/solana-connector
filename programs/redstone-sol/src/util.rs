use zkp_u256::U256;

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

pub trait FromBytesRepr<T: Sized>: Sized {
    fn from_bytes(bytes: T) -> Self;
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

pub fn median(values: &[U256]) -> Option<U256> {
    match values.len() {
        0 => None,
        1 => Some(values[0].clone()),
        2 => Some(avg_u256(&values[0], &values[1])),
        len => {
            // Create a vector of indices
            let mut indices: Vec<usize> = (0..len).collect();

            // Sort the indices based on the values they point to
            indices.sort_unstable_by(|&i, &j| values[i].cmp(&values[j]));

            let mid = len / 2;
            if len % 2 == 0 {
                Some(avg_u256(
                    &values[indices[mid - 1]],
                    &values[indices[mid]],
                ))
            } else {
                Some(values[indices[mid]].clone())
            }
        }
    }
}

pub fn avg_u256(a: &U256, b: &U256) -> U256 {
    if a > b {
        b + (a - b) / U256::from(2u64)
    } else {
        a + (b - a) / U256::from(2u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::time::{Duration, Instant};

    /// checks if no overflow occurs
    #[test]
    fn test_median_with_max_values() {
        let values = vec![U256::MAX; 100];
        let result = median(&values);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), U256::MAX);
    }

    #[test]
    fn test_median_avg() {
        let values = vec![
            U256::MAX,
            U256::MAX,
            U256::MAX - U256::from(10),
            U256::MAX - U256::from(10),
        ];
        let result = median(&values);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), U256::MAX - U256::from(5));
    }

    /// median of u128 implementation for benchmark comparison
    /// RNG fuzz benchmark results:
    /// Total iterations: 1000
    /// Average duration (u128): 17.106µs
    /// Average duration (U256): 25.093µs
    fn median_u128(values: &[u128]) -> Option<u128> {
        match values.len() {
            0 => None,
            1 => Some(values[0]),
            2 => Some(
                values[0] / 2
                    + values[1] / 2
                    + (values[0] % 2 + values[1] % 2) / 2,
            ),
            len => {
                let mut values = values.to_vec();
                values.sort_unstable();
                let mid = len / 2;
                if len % 2 == 0 {
                    Some(
                        values[mid - 1] / 2
                            + values[mid] / 2
                            + (values[mid - 1] % 2 + values[mid] % 2) / 2,
                    )
                } else {
                    Some(values[mid])
                }
            }
        }
    }

    #[test]
    fn benchmark_median_comparison() {
        let iterations = 1000;
        let mut total_duration_u128 = Duration::new(0, 0);
        let mut total_duration_u256 = Duration::new(0, 0);
        let mut rng = rand::thread_rng();

        for _ in 0..iterations {
            // Generate random u128 values
            let values_u128: Vec<u128> =
                (0..100).map(|_| rng.gen::<u128>()).collect();

            // Convert u128 values to U256
            let values_u256: Vec<U256> =
                values_u128.iter().map(|&x| U256::from(x)).collect();

            // Measure u128 median calculation
            let start = Instant::now();
            let median_result_u128 = median_u128(&values_u128);
            total_duration_u128 += start.elapsed();

            // Measure U256 median calculation
            let start = Instant::now();
            let median_result_u256 = median(&values_u256);
            total_duration_u256 += start.elapsed();

            // Compare results
            assert!(
                median_result_u128.is_some() && median_result_u256.is_some()
            );
            let u128_result = median_result_u128.unwrap();
            let u256_result = median_result_u256.unwrap();
            assert_eq!(
                U256::from(u128_result),
                u256_result,
                "Median results don't match"
            );
            // println!(
            //     "u128: {:?}, U256: {:?}, results: u128 {:?} == U256 {:?}",
            //     total_duration_u128,
            //     total_duration_u256,
            //     u128_result.to_string(),
            //     u256_result.to_decimal_str()
            // );
        }

        let avg_duration_u128 = total_duration_u128 / iterations as u32;
        let avg_duration_u256 = total_duration_u256 / iterations as u32;

        println!("RNG fuzz benchmark results:");
        println!("  Total iterations: {}", iterations);
        println!("  Average duration (u128): {:?}", avg_duration_u128);
        println!("  Average duration (U256): {:?}", avg_duration_u256);
    }
}
