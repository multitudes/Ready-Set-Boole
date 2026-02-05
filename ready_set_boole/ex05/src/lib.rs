use ex03::{Node, parse_rpn};

/// Converts a Boolean formula to Negation Normal Form (NNF).
///
/// Transforms a formula in Reverse Polish Notation (RPN) to its equivalent
/// in Negation Normal Form, where negations are only applied to variables.
/// The result contains only the operators: `!`, `&`, and `|`.
///
/// # Arguments
///
/// * `formula` - A string slice containing the RPN expression
///
/// # Returns
///
/// A string representing the formula in NNF and RPN notation
///
/// # Transformations Applied
///
/// - Double negation elimination: `¬¬A = A`
/// - De Morgan's laws: `¬(A ∧ B) = ¬A ∨ ¬B`, `¬(A ∨ B) = ¬A ∧ ¬B`
/// - Material condition: `A ⇒ B = ¬A ∨ B`
/// - Equivalence: `A ⇔ B = (A ∧ B) ∨ (¬A ∧ ¬B)`
/// - XOR: `A ⊕ B = (A ∨ B) ∧ ¬(A ∧ B)`
///
/// # Panics
///
/// Panics if the formula is malformed (invalid RPN syntax)
///
/// # Examples
///
/// ```
/// use ex05::negation_normal_form;
///
/// assert_eq!(negation_normal_form("AB&!"), "A!B!|");
/// assert_eq!(negation_normal_form("AB>"), "A!B|");
/// ```
pub fn negation_normal_form(formula: &str) -> String {
    // Parse RPN to AST
    let tree: Node = match parse_rpn(formula) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error parsing formula: {}", e);
            std::process::exit(1);
        }
    };
    // Transform to NNF
    let nnf_tree = ast_to_nnf(&tree);

    // Convert back to RPN
    ast_to_rpn(&nnf_tree)
}

/// Converts an AST to Negation Normal Form (NNF).
///
/// Recursively applies NNF transformation rules to a parsed formula tree.
/// Negations are pushed down to variables, and `>`, `=`, and `^` are eliminated,
/// so the result contains only variables and the operators `!`, `&`, and `|`.
///
/// # Arguments
///
/// * `node` - The AST node to transform
///
/// # Returns
///
/// A new AST node in Negation Normal Form
///
/// # Transformations Applied
///
/// - Double negation: `¬¬A = A`
/// - De Morgan's laws: `¬(A ∧ B) = ¬A ∨ ¬B`, `¬(A ∨ B) = ¬A ∧ ¬B`
/// - Implication: `A ⇒ B = ¬A ∨ B`
/// - Negated implication: `¬(A ⇒ B) = A ∧ ¬B`
/// - Equivalence: `A ⇔ B = (A ∧ B) ∨ (¬A ∧ ¬B)`
/// - Negated equivalence: `¬(A ⇔ B) = (A ∧ ¬B) ∨ (¬A ∧ B)`
/// - XOR: `A ⊕ B = (A ∨ B) ∧ ¬(A ∧ B)`
///
/// # Examples
///
/// ```
/// use ex05::{to_nnf};
/// use ex03::parse_rpn;
///
/// let tree = parse_rpn("AB&!").unwrap();
/// let nnf = to_nnf(&tree);
/// // nnf now represents A!B!|
/// ```
pub fn ast_to_nnf(node: &Node) -> Node {
    match node {
        // Double negation elimination: ¬¬A = A
        Node::Not(inner) => match &**inner {
            // handle double negation ¬¬A => A
            Node::Not(double) => ast_to_nnf(double),
            // De Morgan's law: ¬(A ∧ B) = ¬A ∨ ¬B
            Node::And(a, b) => {
                let not_a = Node::Not(a.clone());
                let not_b = Node::Not(b.clone());
                ast_to_nnf(&Node::Or(Box::new(not_a), Box::new(not_b)))
            }
            // De Morgan's law: ¬(A ∨ B) = ¬A ∧ ¬B
            Node::Or(a, b) => {
                let not_a = Node::Not(a.clone());
                let not_b = Node::Not(b.clone());
                ast_to_nnf(&Node::And(Box::new(not_a), Box::new(not_b)))
            }
            // ¬(A ⇒ B) = A ∧ ¬B
            Node::Imply(a, b) => {
                let not_b = Node::Not(b.clone());
                ast_to_nnf(&Node::And(a.clone(), Box::new(not_b)))
            }
            // ¬(A ⇔ B) = (A ∧ ¬B) ∨ (¬A ∧ B)
            Node::Equiv(a, b) => {
                let not_a = Node::Not(a.clone());
                let not_b = Node::Not(b.clone());
                let a_and_not_b = ast_to_nnf(&Node::And(a.clone(), Box::new(not_b)));
                let not_a_and_b = Node::And(Box::new(not_a), b.clone());
                ast_to_nnf(&Node::Or(Box::new(a_and_not_b), Box::new(not_a_and_b)))
            }
            // this is supposed to handle the only one left which is XOR
            // for XOR i dont need to rewrite anything so i just pass the inner value to nnf
            _ => Node::Not(Box::new(ast_to_nnf(inner))),
        },

        // leaf values
        Node::Value(b) => Node::Value(*b),
        Node::Variable(c) => Node::Variable(*c),

        // Material condition: A ⇒ B = ¬A ∨ B
        Node::Imply(a, b) => {
            let not_a = Node::Not(a.clone());
            ast_to_nnf(&Node::Or(Box::new(not_a), b.clone()))
        }

        // Equivalence: A ⇔ B = (A ∧ B) ∨ (¬A ∧ ¬B)
        Node::Equiv(a, b) => {
            let and_part = Node::And(a.clone(), b.clone());
            let not_a = Node::Not(a.clone());
            let not_b = Node::Not(b.clone());
            let not_and_part = Node::And(Box::new(not_a), Box::new(not_b));
            ast_to_nnf(&Node::Or(Box::new(and_part), Box::new(not_and_part)))
        }

        // Recursively transform children
        Node::And(a, b) => Node::And(Box::new(ast_to_nnf(a)), Box::new(ast_to_nnf(b))),
        Node::Or(a, b) => Node::Or(Box::new(ast_to_nnf(a)), Box::new(ast_to_nnf(b))),
        // XOR: A ⊕ B = (A ∨ B) ∧ (¬A ∨ ¬B)
        Node::Xor(a, b) => {
            let not_a = Node::Not(a.clone());
            let not_b = Node::Not(b.clone());
            let or_node: Node = Node::Or(a.clone(), b.clone());
            let not_or_node = Node::Or(Box::new(not_a), Box::new(not_b));
            ast_to_nnf(&Node::And(Box::new(or_node), Box::new(not_or_node)))
        }
    }
}

/// Converts an AST back to RPN string representation.
///
/// # Arguments
///
/// * `node` - The AST node
///
/// # Returns
///
/// A string representing the formula in RPN notation
///
/// # Examples
/// ```
/// use ex05::{negation_normal_form};
/// let result = negation_normal_form("AB&!");
/// assert_eq!(result, "A!B!|"); // De Morgan's law applied
/// ```
pub fn ast_to_rpn(node: &Node) -> String {
    match node {
        Node::Value(b) => if *b { "1" } else { "0" }.to_string(),
        Node::Variable(c) => c.to_string(),
        Node::Not(a) => format!("{}!", ast_to_rpn(a)),
        Node::And(..) => {
            let mut operands = vec![];
            collect_and_operands(node, &mut operands);
            let mut result = String::new();
            for op in &operands {
                result.push_str(&ast_to_rpn(&op));
            }
            for _ in 1..operands.len() {
                result.push('&');
            }
            result
        }

        Node::Or(..) => {
            let mut operands = vec![];
            collect_or_operands(node, &mut operands);
            let mut result = String::new();
            for op in &operands {
                result.push_str(&ast_to_rpn(&op));
            }
            for _ in 1..operands.len() {
                result.push('|');
            }
            result
        }

        Node::Xor(a, b) => format!("{}{}^", ast_to_rpn(a), ast_to_rpn(b)),
        Node::Imply(a, b) => format!("{}{}>", ast_to_rpn(a), ast_to_rpn(b)),
        Node::Equiv(a, b) => format!("{}{}=", ast_to_rpn(a), ast_to_rpn(b)),
    }
}

fn collect_or_operands(node: &Node, operands: &mut Vec<Node>) {
    match node {
        Node::Or(a, b) => {
            collect_or_operands(a, operands);
            collect_or_operands(b, operands);
        }
        _ => operands.push(node.clone()),
    }
}

fn collect_and_operands(node: &Node, operands: &mut Vec<Node>) {
    match node {
        Node::And(a, b) => {
            collect_and_operands(a, operands);
            collect_and_operands(b, operands);
        }
        _ => operands.push(node.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = negation_normal_form("AB&!");
        assert_eq!(result, "A!B!|");
    }

    #[test]
    fn test_de_morgans_law_and() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
    }

    #[test]
    fn test_de_morgans_law_or() {
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
    }

    #[test]
    fn test_material_condition() {
        assert_eq!(negation_normal_form("AB>"), "A!B|");
    }

    #[test]
    fn test_equivalence() {
        assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
    }

    #[test]
    fn test_complex_negation_of_or_and() {
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
    }

    #[test]
    fn test_double_negation() {
        assert_eq!(negation_normal_form("A!!"), "A");
        assert_eq!(negation_normal_form("A!!!"), "A!");
    }

    #[test]
    fn test_triple_negation() {
        assert_eq!(negation_normal_form("A!!!"), "A!");
    }

    #[test]
    fn test_complex_implication() {
        assert_eq!(negation_normal_form("AB&C>"), "A!B!C||");
    }

    #[test]
    fn test_negation_of_implication() {
        assert_eq!(negation_normal_form("AB>!"), "AB!&");
    }

    #[test]
    fn test_negation_of_equivalence() {
        assert_eq!(negation_normal_form("AB=!"), "AB!&A!B&|");
    }

    #[test]
    fn test_three_variables_no_change() {
        assert_eq!(negation_normal_form("ABC&|"), "ABC&|");
    }

    #[test]
    fn test_three_variables_de_morgans() {
        assert_eq!(negation_normal_form("ABC&|!"), "A!B!C!|&");
    }

    #[test]
    fn test_complex_nested_1() {
        assert_eq!(negation_normal_form("AB&C|!"), "A!B!|C!&");
    }

    #[test]
    fn test_complex_nested_2() {
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
    }

    #[test]
    fn test_xor_passthrough() {
        let result = negation_normal_form("AB^");
        // println!("Result: {}", result);
        assert_eq!(result, "AB|A!B!|&");
    }

    #[test]
    fn test_constants_only() {
        assert_eq!(negation_normal_form("10&"), "10&");
    }

    #[test]
    fn test_single_variable() {
        assert_eq!(negation_normal_form("A"), "A");
    }

    #[test]
    fn test_single_negation() {
        assert_eq!(negation_normal_form("A!"), "A!");
    }

    #[test]
    fn test_de_morgan_and() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
    }

    #[test]
    fn test_de_morgan_or() {
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
    }

    #[test]
    fn test_implication_to_nnf() {
        // A => B  ->  !A | B
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        // !(A => B) -> A & !B
        assert_eq!(negation_normal_form("AB>!"), "AB!&");
    }

    #[test]
    fn test_complex_nesting() {
        // !( (A & B) | (C & D) ) -> (!A | !B) & (!C | !D)
        let input = "AB&CD&|!";
        let result = negation_normal_form(input);
        assert_eq!(result, "A!B!|C!D!|&");
    }
}
