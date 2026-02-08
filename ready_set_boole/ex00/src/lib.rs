/// Adds two numbers using only bit wise operators.
///
/// This implementation uses bitwise XOR for addition without carry,
/// and bitwise AND with left shift to handle the carry propagation.
///
/// # Arguments
///
/// * `a` - The first number to add
/// * `b` - The second number to add
///
/// # Returns
///
/// The sum of `a` and `b`
///
/// # Algorithm
///
/// For each iteration:
/// - Calculate carry: `a & b`
/// - Update a: `a ^ b` (sum without carry)
/// - Shift carry left: `carry << 1`
/// - Repeat until no carry remains
///
/// # Examples
/// ```
/// use ex00::adder;
///
/// let answer = adder(2, 2);
/// assert_eq!(4, answer);
/// ```
pub fn adder(mut a: u32, mut b: u32) -> u32 {
    while b > 0 {
        let carry = a & b;
        a = a ^ b;
        b = carry << 1;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_addition() {
        assert_eq!(adder(1, 2), 3);
        assert_eq!(adder(13, 37), 50);
    }

    #[test]
    fn test_zero_cases() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(5, 0), 5);
        assert_eq!(adder(0, 10), 10);
    }

    #[test]
    fn test_larger_numbers() {
        assert_eq!(adder(100, 200), 300);
        assert_eq!(adder(1000, 5000), 6000);
    }

    #[test]
    fn test_powers_of_two() {
        assert_eq!(adder(8, 4), 12);
        assert_eq!(adder(16, 32), 48);
    }

    #[test]
    fn test_large_u32_values() {
        assert_eq!(adder(u32::MAX - 1, 1), u32::MAX);
        assert_eq!(adder(1000000, 2000000), 3000000);
    }

    #[test]
    fn test_commutative_property() {
        assert_eq!(adder(7, 3), adder(3, 7));
        assert_eq!(adder(7, 3), 10);
    }
}
