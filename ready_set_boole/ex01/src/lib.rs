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
    }

    #[test]
    fn test_basic_multiplication() {
        assert_eq!(multiplier(1, 2), 2);
        assert_eq!(multiplier(3, 4), 12);
        assert_eq!(multiplier(5, 7), 35);
    }

    #[test]
    fn test_zero_cases() {
        assert_eq!(multiplier(0, 37), 0);
        assert_eq!(multiplier(0, 0), 0);
        assert_eq!(multiplier(15, 0), 0);
    }

    #[test]
    fn test_one_cases() {
        assert_eq!(multiplier(1, 1), 1);
        assert_eq!(multiplier(1, 100), 100);
        assert_eq!(multiplier(50, 1), 50);
    }

    #[test]
    fn test_powers_of_two() {
        assert_eq!(multiplier(2, 2), 4);
        assert_eq!(multiplier(8, 4), 32);
        assert_eq!(multiplier(16, 16), 256);
    }

    #[test]
    fn test_larger_numbers() {
        assert_eq!(multiplier(100, 100), 10000);
        assert_eq!(multiplier(1000, 2), 2000);
        assert_eq!(multiplier(123, 456), 56088);
    }

    #[test]
    fn test_commutative_property() {
        assert_eq!(multiplier(7, 6), multiplier(6, 7));
        assert_eq!(multiplier(7, 6), 42);
    }
}
