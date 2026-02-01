use ex00::adder;

/// Multiplies two numbers using only bitwise operators.
///
/// This implementation uses binary multiplication (similar to long multiplication)
/// with bitwise operations: for each bit set in `b`, the corresponding shifted
/// value of `a` is added to the result using bitwise addition.
///
/// # Arguments
///
/// * `a` - The first number to multiply
/// * `b` - The second number to multiply
///
/// # Returns
///
/// The product of `a` and `b`
///
/// # Algorithm
///
/// Binary multiplication works like decimal long multiplication:
/// - For each bit position in `b`:
///   - If the bit is 1, add the shifted value of `a` to the result
///   - Shift `a` left for the next bit position
///   - Shift `b` right to check the next bit
/// - Uses bitwise addition (XOR + carry propagation) to accumulate the result
///
/// # Examples
/// ```
/// use ex01::multiplier;
/// let answer = multiplier(3, 4);
///
/// assert_eq!(12, answer);
/// ```
pub fn multiplier(mut a: u32, mut b: u32) -> u32 {
    let mut result = 0;

    while b > 0 {
        if b & 1 == 1 {
            result = adder(result, a);
        }
        a <<= 1;
        b >>= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = multiplier(3, 4);
        assert_eq!(result, 12);
        let result = multiplier(3, 1);
        assert_eq!(result, 3);
        let result = multiplier(0, 4);
        assert_eq!(result, 0);
        let result = multiplier(3, 0);
        assert_eq!(result, 0);
    }
}
