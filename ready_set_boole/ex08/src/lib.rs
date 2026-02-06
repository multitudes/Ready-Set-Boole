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
