/// Generates the powerset of a given set using binary enumeration.
///
/// The powerset of a set S is the set of all possible subsets of S, including
/// the empty set and S itself. For a set with n elements, the powerset contains
/// exactly 2^n subsets.
///
/// This implementation uses the **binary enumeration method**, where each subset
/// corresponds to a binary number from 0 to 2^n - 1. Each bit position indicates
/// whether the corresponding element is included in that subset.
///
/// # Arguments
///
/// * `set` - A vector of integers representing the input set
///
/// # Returns
///
/// A vector of vectors, where each inner vector is a subset of the input set.
/// The subsets are ordered according to their binary representation (lexicographic order).
///
/// # Algorithm
///
/// 1. **Count subsets**: Calculate 2^n where n is the number of elements
/// 2. **Iterate through binary numbers**: For each number i from 0 to 2^n - 1
/// 3. **Check each bit**: For each bit position j in i
/// 4. **Include element**: If bit j is set (1), include element j in the subset
/// 5. **Collect subsets**: Add each generated subset to the result
///
/// # Binary Representation
///
/// For a set [A, B, C], the subsets correspond to binary numbers:
/// ```text
/// Binary  Decimal  Subset
/// 000   →   0    → []
/// 001   →   1    → [A]
/// 010   →   2    → [B]
/// 011   →   3    → [A, B]
/// 100   →   4    → [C]
/// 101   →   5    → [A, C]
/// 110   →   6    → [B, C]
/// 111   →   7    → [A, B, C]
/// ```
///
/// # Time Complexity
///
/// O(n × 2^n) where n is the number of elements in the input set:
/// - Outer loop: 2^n iterations
/// - Inner loop: n bit checks per iteration
///
/// # Space Complexity
///
/// O(n × 2^n) to store all subsets
///
/// # Examples
///
/// ```
/// use ex08::powerset;
///
/// // Empty set
/// assert_eq!(powerset(vec![]), vec![vec![]]);
///
/// // Single element
/// assert_eq!(powerset(vec![1]), vec![vec![], vec![1]]);
///
/// // Two elements
/// assert_eq!(
///     powerset(vec![1, 2]),
///     vec![vec![], vec![1], vec![2], vec![1, 2]]
/// );
///
/// // Three elements - 8 subsets total
/// let result = powerset(vec![1, 2, 3]);
/// assert_eq!(result.len(), 8);  // 2^3 = 8
/// assert_eq!(result[0], vec![]);           // Binary 000
/// assert_eq!(result[7], vec![1, 2, 3]);    // Binary 111
/// ```
///
/// # Mathematical Properties
///
/// The powerset forms a **Boolean lattice** where:
/// - The empty set {} is the bottom element (⊥)
/// - The full set is the top element (⊤)
/// - Subset relation (⊆) defines the partial order
/// - Union (∪) is the join operation
/// - Intersection (∩) is the meet operation
///
/// # Notes
///
/// - The ordering of subsets follows binary enumeration (not lexicographic order of elements)
/// - For large sets (n > 20), this becomes computationally expensive (2^20 = 1,048,576 subsets)
/// - This is the optimal algorithm for generating all subsets (can't do better than O(n × 2^n))
pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let n = set.len();
    let num_subset = 1 << n;
    let mut results = Vec::with_capacity(num_subset);

    for i in 0..num_subset {
        let mut subset: Vec<i32> = Vec::new();
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                subset.push(set[j]);
            }
        }
        results.push(subset);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let set: Vec<i32> = vec![];
        let result = powerset(set);
        println!("Powerset of []:");
        for subset in &result {
            println!("  {:?}", subset);
        }
        assert_eq!(result, vec![vec![]]);
        assert_eq!(result.len(), 1); // 2^0 = 1
    }

    #[test]
    fn test_single() {
        let set = vec![1];
        let result = powerset(set);
        println!("\nPowerset of [1]:");
        for subset in &result {
            println!("  {:?}", subset);
        }
        assert_eq!(result, vec![vec![], vec![1]]);
        assert_eq!(result.len(), 2); // 2^1 = 2
    }

    #[test]
    fn test_two_elements() {
        let set = vec![1, 2];
        let result = powerset(set);
        println!("\nPowerset of [1, 2]:");
        for subset in &result {
            println!("  {:?}", subset);
        }
        let expected = vec![vec![], vec![1], vec![2], vec![1, 2]];
        assert_eq!(result, expected);
        assert_eq!(result.len(), 4); // 2^2 = 4
    }

    #[test]
    fn test_three_elements() {
        let set = vec![1, 2, 3];
        let result = powerset(set);
        println!("\nPowerset of [1, 2, 3]:");
        for subset in &result {
            println!("  {:?}", subset);
        }
        let expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        assert_eq!(result, expected);
        assert_eq!(result.len(), 8); // 2^3 = 8
    }

    #[test]
    fn test_four_elements() {
        let set = vec![1, 2, 3, 4];
        let result = powerset(set);
        println!("\nPowerset of [1, 2, 3, 4] (length: {}):", result.len());
        assert_eq!(result.len(), 16); // 2^4 = 16
    }

    #[test]
    fn test_order_property() {
        // Verify that subsets are generated in binary order
        let set = vec![10, 20, 30];
        let result = powerset(set);
        println!("Powerset of [10, 20, 30]:");
        for subset in &result {
            println!("  {:?}", subset);
        }
        // Element 0: binary 000 -> []
        assert_eq!(result[0], vec![]);
        // Element 1: binary 001 -> [10]
        assert_eq!(result[1], vec![10]);
        // Element 2: binary 010 -> [20]
        assert_eq!(result[2], vec![20]);
        // Element 3: binary 011 -> [10, 20]
        assert_eq!(result[3], vec![10, 20]);
        // Element 4: binary 100 -> [30]
        assert_eq!(result[4], vec![30]);
        // Element 5: binary 101 -> [10, 30]
        assert_eq!(result[5], vec![10, 30]);
        // Element 6: binary 110 -> [20, 30]
        assert_eq!(result[6], vec![20, 30]);
        // Element 7: binary 111 -> [10, 20, 30]
        assert_eq!(result[7], vec![10, 20, 30]);
    }
}
