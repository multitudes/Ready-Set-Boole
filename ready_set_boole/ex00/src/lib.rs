
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
/// let answer = adder(2, 2);
///
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
    fn it_works() {
        let result = adder(2, 2);
        assert_eq!(result, 4);
    }
}
