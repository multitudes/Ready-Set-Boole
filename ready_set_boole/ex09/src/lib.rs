use ex03::{Node, parse_rpn};
use ex04::get_variables;

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    let tree: Node = match parse_rpn(formula) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error parsing formula: {}", e);
            return vec![];
        }
    };
    let mut universe = Vec::new();
    for s in &sets {
        universe.extend(s.clone());
    }
    let universe = clean(universe);
    let variables = get_variables(formula);
    if variables.len() != sets.len() {
        eprintln!(
            "Warning: Formula has {} variables, but {} sets provided. Extra sets will be ignored.",
            variables.len(),
            sets.len()
        );
    }
    evaluate_node(&tree, &sets, &universe)
}

/// Helper to ensure the Vector behaves like a mathematical Set
fn clean(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v.dedup();
    v
}

fn evaluate_node(node: &Node, sets: &Vec<Vec<i32>>, universe: &Vec<i32>) -> Vec<i32> {
    match node {
        // Variables: Map 'A' -> sets[0], 'B' -> sets[1], etc.
        Node::Variable(c) => {
            let idx = (*c as usize) - ('A' as usize);
            sets.get(idx).cloned().unwrap_or_default()
        }

        // Constants: True is the Universe, False is the Empty Set
        Node::Value(b) => {
            if *b {
                universe.clone()
            } else {
                vec![]
            }
        }

        // NOT (Complement): Elements in Universe NOT in child
        Node::Not(a) => {
            let sub = evaluate_node(a, sets, universe);
            universe
                .iter()
                .filter(|x| !sub.contains(x))
                .cloned()
                .collect()
        }

        // AND (Intersection): Elements present in both sets
        Node::And(l, r) => {
            let left = evaluate_node(l, sets, universe);
            let right = evaluate_node(r, sets, universe);
            left.into_iter().filter(|x| right.contains(x)).collect()
        }

        // OR (Union): Merge both and clean
        Node::Or(l, r) => {
            let mut left = evaluate_node(l, sets, universe);
            let right = evaluate_node(r, sets, universe);
            left.extend(right);
            clean(left)
        }

        // XOR (Symmetric Difference): Elements in one or the other, but not both
        Node::Xor(a, b) => {
            let left = evaluate_node(a, sets, universe);
            let right = evaluate_node(b, sets, universe);
            let l_not_r = left.iter().filter(|x| !right.contains(x));
            let r_not_l = right.iter().filter(|x| !left.contains(x));
            clean(l_not_r.chain(r_not_l).cloned().collect())
        }

        // IMPLY: !A | B
        Node::Imply(a, b) => {
            let left = evaluate_node(a, sets, universe);
            let right = evaluate_node(b, sets, universe);
            let mut not_a: Vec<i32> = universe
                .iter()
                .filter(|x| !left.contains(x))
                .cloned()
                .collect();
            not_a.extend(right);
            clean(not_a)
        }

        // EQUIVALENT: (A & B) | (!A & !B)
        Node::Equiv(a, b) => {
            let left = evaluate_node(a, sets, universe);
            let right = evaluate_node(b, sets, universe);
            let both: Vec<i32> = left.iter().filter(|x| right.contains(x)).cloned().collect();
            let neither: Vec<i32> = universe
                .iter()
                .filter(|x| !left.contains(x) && !right.contains(x))
                .cloned()
                .collect();
            clean(both.into_iter().chain(neither).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complement() {
        // A = {1, 2}, B = {3, 4}
        // Universe = {1, 2, 3, 4}
        // A! = {3, 4}
        let sets = vec![vec![1, 2], vec![3, 4]];
        let result = eval_set("A!", sets);
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    fn test_complement_b() {
        // A = {1, 2}, B = {3, 4}
        // Universe = {1, 2, 3, 4}
        // B! = {1, 2}
        let sets = vec![vec![1, 2], vec![3, 4]];
        let result = eval_set("B!", sets);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_xor() {
        // A = {1, 2, 3}, B = {3, 4, 5}
        // Universe = {1, 2, 3, 4, 5}
        // AB^ = {1, 2, 4, 5} (in one or the other, but not both)
        let sets = vec![vec![1, 2, 3], vec![3, 4, 5]];
        let result = eval_set("AB^", sets);
        assert_eq!(result, vec![1, 2, 4, 5]);
    }

    #[test]
    fn test_imply() {
        // A = {1, 2}, B = {2, 3}
        // Universe = {1, 2, 3}
        // AB> = !A | B = {2, 3}
        let sets = vec![vec![1, 2], vec![2, 3]];
        let result = eval_set("AB>", sets);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_intersection() {
        // A = {1, 2}, B = {1, 3}, C = {4}
        // Universe = {1, 2, 3, 4}
        // AB& = {1}
        let sets = vec![vec![1, 2], vec![1, 3], vec![4]];
        let result = eval_set("AB&", sets);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_equivalence() {
        // A = {1, 2}, B = {1, 3}, C = {4}
        // Universe = {1, 2, 3, 4}
        // AB= = (A & B) | (!A & !B) = {1} | {4} = {1, 4}
        let sets = vec![vec![1, 2], vec![1, 3], vec![4]];
        let result = eval_set("AB=", sets);
        assert_eq!(result, vec![1, 4]);
    }

    #[test]
    fn test_complex_and_or() {
        // A = {1, 2}, B = {2, 3}, C = {3, 4}
        // AB&C| = (A & B) | C = {2} | {3, 4} = {2, 3, 4}
        let sets = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        let result = eval_set("AB&C|", sets);
        assert_eq!(result, vec![2, 3, 4]);
    }

    #[test]
    fn test_union() {
        // A = {1, 2}, B = {2, 3}
        // Universe = {1, 2, 3}
        // AB| = {1, 2, 3}
        let sets = vec![vec![1, 2], vec![2, 3]];
        let result = eval_set("AB|", sets);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_constant_true() {
        // Formula: '1' (just the constant true)
        // A = {1, 2}
        // Result should be the universe {1, 2}
        let sets = vec![vec![1, 2]];
        let result = eval_set("1", sets);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_constant_false() {
        // Formula: '0' (just the constant false)
        // A = {1, 2}
        // Result should be empty set {}
        let sets = vec![vec![1, 2]];
        let result = eval_set("0", sets);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_empty_sets() {
        // A = {}, B = {}
        // Universe = {}
        // AB| = {}
        let sets = vec![vec![], vec![]];
        let result = eval_set("AB|", sets);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_de_morgans_and() {
        // A = {1, 2}, B = {2, 3}
        // Universe = {1, 2, 3}
        // !(A & B) = !A | !B
        let sets_neg = vec![vec![1, 2], vec![2, 3]];
        let sets_demorgan = vec![vec![1, 2], vec![2, 3]];

        let result1 = eval_set("AB&!", sets_neg);
        let result2 = eval_set("A!B!|", sets_demorgan);

        assert_eq!(result1, result2);
        assert_eq!(result1, vec![1, 3]);
    }

    #[test]
    fn test_de_morgans_or() {
        // A = {1, 2}, B = {2, 3}
        // Universe = {1, 2, 3}
        // !(A | B) = !A & !B
        let sets_neg = vec![vec![1, 2], vec![2, 3]];
        let sets_demorgan = vec![vec![1, 2], vec![2, 3]];

        let result1 = eval_set("AB|!", sets_neg);
        let result2 = eval_set("A!B!&", sets_demorgan);

        assert_eq!(result1, result2);
        assert_eq!(result1, vec![]);
    }

    #[test]
    fn test_single_variable() {
        // Formula: 'A'
        // A = {1, 2, 3}
        // Result should be A itself
        let sets = vec![vec![1, 2, 3]];
        let result = eval_set("A", sets);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_double_negation() {
        // A = {1, 2}
        // Universe = {1, 2, 3}
        // A!! should equal A
        let sets = vec![vec![1, 2]];
        let result = eval_set("A!!", sets);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_three_variable_complex() {
        // A = {1, 2}, B = {2, 3}, C = {1, 3}
        // Universe = {1, 2, 3}
        // (A | B) & C = {1, 2, 3} & {1, 3} = {1, 3}
        let sets = vec![vec![1, 2], vec![2, 3], vec![1, 3]];
        let result = eval_set("AB|C&", sets);
        assert_eq!(result, vec![1, 3]);
    }
}
