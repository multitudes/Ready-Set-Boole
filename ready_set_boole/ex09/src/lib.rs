use ex03::{Node, parse_rpn};

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    let tree: Node = match parse_rpn(formula) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error parsing formula: {}", e);
            std::process::exit(1);
        }
    };
    let mut universe = Vec::new();
    for s in &sets {
        universe.extend(s.clone());
    }
    let universe = clean(universe);
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
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
