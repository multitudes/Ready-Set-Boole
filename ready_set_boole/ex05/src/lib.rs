
use ex03::Node;
use ex03::parse_rpn;

pub fn negation_normal_form(formula: &str) -> String {
    // Parse RPN to AST
    let tree = match parse_rpn(formula) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error parsing formula: {}", e);
            std::process::exit(1);
        }
    };
    // Transform to NNF
    let nnf_tree = to_nnf(&tree);
    
    // Convert back to RPN
    ast_to_rpn(&nnf_tree)
}

/// Transforms an AST to Negation Normal Form.
///
/// Applies the following transformations:
/// - Double negation elimination: ¬¬A = A
/// - De Morgan's laws: ¬(A ∧ B) = ¬A ∨ ¬B, ¬(A ∨ B) = ¬A ∧ ¬B
/// - Material condition: A ⇒ B = ¬A ∨ B
/// - Equivalence: A ⇔ B = (A ∧ B) ∨ (¬A ∧ ¬B)
///
/// Result contains only: variables, !, &, and |
fn to_nnf(node: &Node) -> Node {
    match node {
        Node::Value(b) => Node::Value(*b),
        Node::Variable(c) => Node::Variable(*c),
        
        // Double negation elimination: ¬¬A = A
        Node::Not(inner) => match &**inner {
            Node::Not(double) => to_nnf(double),
            // De Morgan's law: ¬(A ∧ B) = ¬A ∨ ¬B
            Node::And(a, b) => {
                let not_a = Node::Not(Box::new(to_nnf(a)));
                let not_b = Node::Not(Box::new(to_nnf(b)));
                Node::Or(Box::new(not_a), Box::new(not_b))
            }
            // De Morgan's law: ¬(A ∨ B) = ¬A ∧ ¬B
            Node::Or(a, b) => {
                let not_a = Node::Not(Box::new(to_nnf(a)));
                let not_b = Node::Not(Box::new(to_nnf(b)));
                Node::And(Box::new(not_a), Box::new(not_b))
            }
            // ¬(A ⇒ B) = A ∧ ¬B
            Node::Imply(a, b) => {
                let not_b = Node::Not(Box::new(to_nnf(b)));
                Node::And(Box::new(to_nnf(a)), Box::new(not_b))
            }
            // ¬(A ⇔ B) = (A ∧ ¬B) ∨ (¬A ∧ B)
            Node::Equiv(a, b) => {
                let a_nnf = to_nnf(a);
                let b_nnf = to_nnf(b);
                let a_and_not_b = Node::And(
                    Box::new(a_nnf.clone()),
                    Box::new(Node::Not(Box::new(b_nnf.clone())))
                );
                let not_a_and_b = Node::And(
                    Box::new(Node::Not(Box::new(a_nnf))),
                    Box::new(b_nnf)
                );
                Node::Or(Box::new(a_and_not_b), Box::new(not_a_and_b))
            }
            other => Node::Not(Box::new(to_nnf(other))),
        },
        
        // Material condition: A ⇒ B = ¬A ∨ B
        Node::Imply(a, b) => {
            let not_a = Node::Not(Box::new(to_nnf(a)));
            Node::Or(Box::new(not_a), Box::new(to_nnf(b)))
        }
        
        // Equivalence: A ⇔ B = (A ∧ B) ∨ (¬A ∧ ¬B)
        Node::Equiv(a, b) => {
            let a_nnf = to_nnf(a);
            let b_nnf = to_nnf(b);
            let and_part = Node::And(Box::new(a_nnf.clone()), Box::new(b_nnf.clone()));
            let not_and_part = Node::And(
                Box::new(Node::Not(Box::new(a_nnf))),
                Box::new(Node::Not(Box::new(b_nnf)))
            );
            Node::Or(Box::new(and_part), Box::new(not_and_part))
        }
        
        // Recursively transform children
        Node::And(a, b) => Node::And(
            Box::new(to_nnf(a)),
            Box::new(to_nnf(b))
        ),
        Node::Or(a, b) => Node::Or(
            Box::new(to_nnf(a)),
            Box::new(to_nnf(b))
        ),
        Node::Xor(a, b) => Node::Xor(
            Box::new(to_nnf(a)),
            Box::new(to_nnf(b))
        ),
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
        Node::And(a, b) => format!("{}{}&", ast_to_rpn(a), ast_to_rpn(b)),
        Node::Or(a, b) => format!("{}{}|", ast_to_rpn(a), ast_to_rpn(b)),
        Node::Xor(a, b) => format!("{}{}^", ast_to_rpn(a), ast_to_rpn(b)),
        Node::Imply(a, b) => format!("{}{}>", ast_to_rpn(a), ast_to_rpn(b)),
        Node::Equiv(a, b) => format!("{}{}=", ast_to_rpn(a), ast_to_rpn(b)),
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
}
