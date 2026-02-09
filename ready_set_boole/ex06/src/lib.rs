use ex03::{Node, parse_rpn};
use ex05::{ast_to_nnf, ast_to_rpn};

/// Converts a Boolean formula to Conjunctive Normal Form (CNF).
///
/// Transforms a propositional formula in RPN notation to an equivalent CNF formula,
/// where negations appear only directly after variables and the formula is structured
/// as an AND of OR clauses (Product of Sums): $(C_1) \land (C_2) \land \ldots \land (C_n)$.
///
/// # Arguments
///
/// * `formula` - A valid RPN formula string (e.g., `"AB&C|"`)
///
/// # Returns
///
/// An equivalent CNF formula in RPN notation
///
/// # Panics
///
/// Panics if the input formula is not valid RPN or contains parse errors.
///
/// # Algorithm
///
/// 1. **Parse**: Convert RPN string to Abstract Syntax Tree (AST)
/// 2. **NNF**: Transform to Negation Normal Form (push negations to variables)
/// 3. **Distribute**: Apply distributivity law: $A \lor (B \land C) \equiv (A \lor B) \land (A \lor C)$
/// 4. **Flatten**: Convert AST back to RPN string
///
/// # Examples
///
/// ```
/// use ex06::conjunctive_normal_form;
///
/// // Negated AND becomes OR of negations (De Morgan's Law)
/// assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
///
/// // Distribution: (A & B) | C  →  (A | C) & (B | C)
/// assert_eq!(conjunctive_normal_form("AB&C|"), "AC|BC|&");
///
/// // Already in CNF (unchanged)
/// assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
/// ```
///
/// # Mathematical Properties
///
/// **Distributivity Law (core transformation):**
/// ```text
/// A ∨ (B ∧ C) ⟺ (A ∨ B) ∧ (A ∨ C)
/// ```
///
/// **CNF Structure:**
/// - Outer operators: AND (conjunction) - the "Product"
/// - Inner operators: OR (disjunction) - the "Sums"
/// - Negations: Only on variables, never on compound expressions
///
/// **Time Complexity:** O(2^n) worst case due to exponential growth from distribution
///
/// **Applications:**
/// - SAT solvers (standard input format)
/// - Hardware verification
/// - Automated theorem proving
/// - Constraint satisfaction problems
pub fn conjunctive_normal_form(formula: &str) -> String {
    // Parse RPN to AST
    let tree: Node = match parse_rpn(formula) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error parsing formula: {}", e);
            std::process::exit(1);
        }
    };
    // alternate production code
    // let tree = parse_rpn(formula).expect("Failed to parse formula");
    // Transform to NNF
    let nnf_tree = ast_to_nnf(&tree);
    // Transform to CNF
    let cnf_tree = to_cnf(nnf_tree);
    // Convert back to RPN
    ast_to_rpn(&cnf_tree)
}

fn to_cnf(node: Node) -> Node {
    match node {
        Node::And(l, r) => Node::And(Box::new(to_cnf(*l)), Box::new(to_cnf(*r))),
        Node::Or(l, r) => distribute(to_cnf(*l), to_cnf(*r)),
        _ => node,
    }
}

fn distribute(l: Node, r: Node) -> Node {
    match (l, r) {
        // Case: A | (B & C)  ->  (A | B) & (A | C)
        (a, Node::And(b, c)) => Node::And(
            Box::new(distribute(a.clone(), *b)),
            Box::new(distribute(a, *c)),
        ),
        // Case: (A & B) | C  ->  (A | C) & (B | C)
        (Node::And(a, b), c) => Node::And(
            Box::new(distribute(*a, c.clone())),
            Box::new(distribute(*b, c)),
        ),
        (l, r) => Node::Or(Box::new(l), Box::new(r)),
    }
}

/// to do - bonus with the karnaugh maps
/// https://en.wikipedia.org/wiki/Karnaugh_map
/// but this is really complicated
// fn simplified_cnf(formula: &str) -> String {
//     // Parse RPN to AST
//     let tree: Node = match parse_rpn(formula) {
//         Ok(t) => t,
//         Err(e) => {
//             eprintln!("Error parsing formula: {}", e);
//             std::process::exit(1);
//         }
//     };
//     // Transform to NNF
//     let nnf_tree = ast_to_nnf(&tree);
//     // Transform to CNF
//     let cnf_tree = to_cnf(nnf_tree);

//     // [...] to do what are the steps?

//     // Convert back to RPN
//     ast_to_rpn(&cnf_tree)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cnf_examples_from_subject() {
        assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
        assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
        assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
        assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
        assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
        assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
        assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
    }

    #[test]
    fn test_simple_cnf() {
        // A & B is already CNF
        let res = conjunctive_normal_form("AB&");
        println!("CNF(AB&) = {}", res);
        assert_eq!(res, "AB&");
    }

    #[test]
    fn test_distribution_simple() {
        // (A & B) | C  => (A | C) & (B | C)
        let res = conjunctive_normal_form("AB&C|");
        // Note: depending on your logic, it might be AC|BC|& or CA|CB|&
        assert!(res == "AC|BC|&" || res == "CA|CB|&");
    }

    #[test]
    fn test_negated_or_to_cnf() {
        // !(A | B) => !A & !B (Which is already CNF)
        assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
    }

    #[test]
    fn test_full_distributivity() {
        // (A & B) | (C & D)
        let res = conjunctive_normal_form("AB&CD&|");
        // Verify it contains 4 OR clauses connected by 3 ANDs
        // A valid result would be: AC|AD|&BC|BD|&&
        assert!(res.contains('&'));
        assert_eq!(res.matches('|').count(), 4);
        assert_eq!(res.matches('&').count(), 3);
    }

    #[test]
    fn test_cnf_idempotent_or_clause() {
        // (A | B) is already a clause
        let res = conjunctive_normal_form("AB|");
        assert_eq!(res, "AB|");
    }

    #[test]
    fn test_cnf_nested_and_or() {
        // A | (B & C) => (A | B) & (A | C)
        let res = conjunctive_normal_form("ABC&|");
        assert!(res == "AB|AC|&" || res == "BA|CA|&");
    }

    #[test]
    fn test_cnf_double_distribution() {
        // (A & B) | (C & D) => (A|C)&(A|D)&(B|C)&(B|D)
        let res = conjunctive_normal_form("AB&CD&|");
        assert_eq!(res.matches('|').count(), 4);
        assert_eq!(res.matches('&').count(), 3);
    }

    #[test]
    fn test_cnf_implication() {
        // A => B = !A | B (already CNF)
        let res = conjunctive_normal_form("AB>");
        assert!(res == "A!B|" || res == "B|A!"); // ordering may vary
    }

    #[test]
    fn test_cnf_equivalence() {
        // A <=> B = (A & B) | (!A & !B) -> CNF after distribution
        let res = conjunctive_normal_form("AB=");
        assert!(res.contains('&'));
        assert!(res.contains('|'));
    }

    #[test]
    fn test_cnf_xor() {
        // A XOR B = (A | B) & (!A | !B)
        let res = conjunctive_normal_form("AB^");
        assert!(res == "AB|A!B!|&" || res == "BA|A!B!|&");
    }

    #[test]
    fn test_cnf_constants() {
        assert_eq!(conjunctive_normal_form("1"), "1");
        assert_eq!(conjunctive_normal_form("0"), "0");
        assert_eq!(conjunctive_normal_form("10&"), "10&");
        assert_eq!(conjunctive_normal_form("10|"), "10|");
    }

    #[test]
    fn test_cnf_three_vars_complex() {
        // (A | B) & (C | D) already CNF
        let res = conjunctive_normal_form("AB|CD|&");
        assert!(res == "AB|CD|&" || res == "BA|DC|&");
    }

    #[test]
    fn test_cnf_negated_and() {
        // !(A & B) => !A | !B
        let res = conjunctive_normal_form("AB&!");
        assert_eq!(res, "A!B!|");
    }
}
