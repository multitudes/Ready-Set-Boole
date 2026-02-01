
/// Converts a binary number to its Gray code representation using bitwise operators.
///
/// Gray code is defined as $g = n \oplus (n \gg 1)$, where each successive value
/// differs by only one bit from the previous one.
///
/// # Arguments
///
/// * `n` - The number to convert to Gray code
///
/// # Returns
///
/// The Gray code representation of `n`
///
/// # Algorithm
///
/// - Right shift `n` by 1
/// - XOR the shifted value with the original `n`
///
/// # Examples
/// ```
/// use ex02::gray_code;
/// let answer = gray_code(5); // 5 (101) -> 7 (111)
///
/// assert_eq!(7, answer);
/// ```
pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gray_code_mappings() {
        let cases = [
            (0, 0),
            (1, 1),
            (2, 3),
            (3, 2),
            (4, 6),
            (5, 7),
            (6, 5),
        ];

        for (input, expected) in cases {
            assert_eq!(gray_code(input), expected, "n = {}", input);
        }
    }
}
