

/// Evaluates a Boolean formula in Reverse Polish Notation (RPN).
///
/// The formula consists of:
/// - '1' (True) and '0' (False)
/// - '!' (NOT) - Negates the last element
/// - '&' (AND), '|' (OR), '^' (XOR), '>' (IMPLY), '=' (EQUIVALENT)
///
/// # Arguments
///
/// * `formula` - A string slice containing the RPN expression
///
/// # Returns
///
/// The boolean result of the evaluation
///
/// # Panics
///
/// If the formula is malformed (e.g., not enough operands for an operator) the function will give an error message and return false.
///
/// # RPN
/// Evaluates a Boolean expression in Reverse Polish Notation (RPN).
///
/// Processing logic:
/// 1. Operands (0, 1) are pushed onto a stack.
/// 2. Operators (!, &, |, ^, >, =) pop required operands and push the result.
/// 3. The final value remaining on the stack is the result.
///
/// # Examples
/// ```
/// use ex03::eval_formula;
/// assert_eq!(eval_formula("10&"), false);
/// assert_eq!(eval_formula("10|"), true);
/// assert_eq!(eval_formula("11>"), true); // 1 implies 1 is true
/// ```
pub fn eval_formula(formula: &str) -> bool {
    match parse_rpn(formula) {
        Ok(tree) => eval_node(&tree),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}


/// Abstract Syntax Tree (AST) node for Boolean formulas.
///
/// Represents the structure of a parsed Boolean expression, where:
/// - `Value` is a constant (true or false)
/// - Other variants represent operations with their operands
#[derive(Debug)]
enum Node {
    /// A boolean constant: true (1) or false (0)
    Value(bool),
    /// Logical NOT: ¬a
    Not(Box<Node>),  
    /// Logical AND: a ∧ b
    And(Box<Node>, Box<Node>), 
    /// Logical OR: a ∨ b
    Or(Box<Node>, Box<Node>),  
    /// Logical XOR: a ⊕ b
    Xor(Box<Node>, Box<Node>),
    /// Material implication: a ⇒ b
    Imply(Box<Node>, Box<Node>),
    /// Logical equivalence: a ⇔ b
    Equiv(Box<Node>, Box<Node>),    
}



/// Parses an RPN formula string into an AST.
///
/// Returns a root_node
fn parse_rpn(formula: &str) -> Result<Node, String> {
    let mut stack: Vec<Node> = Vec::new();

    for c in formula.chars() {
        match c {
            '0' | '1' => stack.push(Node::Value(c == '1')),
            '!' => {
                let operand = stack.pop()
                    .ok_or("Error: '!' operator requires 1 operand.")?;
                stack.push(Node::Not(Box::new(operand)));
            }
            '&' | '|' | '^' | '>' | '=' => {
                let b = stack.pop()
                    .ok_or(format!("Error: '{}' operator requires 2 operands.", c))?;
                let a = stack.pop()
                    .ok_or(format!("Error: '{}' operator requires 2 operands.", c))?;
                let node = match c {
                    '&' => Node::And(Box::new(a), Box::new(b)),
                    '|' => Node::Or(Box::new(a), Box::new(b)),
                    '^' => Node::Xor(Box::new(a), Box::new(b)),
                    '>' => Node::Imply(Box::new(a), Box::new(b)),
                    '=' => Node::Equiv(Box::new(a), Box::new(b)),
                    _ => unreachable!(),
                };
                stack.push(node);
            }
            c if c.is_whitespace() => continue,
            _ => return Err(format!("Error: Invalid character '{}' in formula.", c)),
        }
    }

    if stack.len() != 1 {
        return Err(format!("Error: Invalid RPN sequence (stack size is {} at end).", stack.len()));
    }
    Ok(stack.pop().unwrap())
}

/// Recursively evaluates an AST node.
fn eval_node(node: &Node) -> bool {
    match node {
        Node::Value(b) => *b,
        Node::Not(a) => !eval_node(a),
        Node::And(a, b) => eval_node(a) & eval_node(b),
        Node::Or(a, b) => eval_node(a) | eval_node(b),
        Node::Xor(a, b) => eval_node(a) ^ eval_node(b),
        Node::Imply(a, b) => !eval_node(a) | eval_node(b), // A => B is !A | B
        Node::Equiv(a, b) => eval_node(a) == eval_node(b),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_operator() {
        assert_eq!(eval_formula("10&"), false); // 1 AND 0 = false
        assert_eq!(eval_formula("11&"), true); // 1 AND 0 = false
        assert_eq!(eval_formula("01&"), false); // 1 AND 0 = false
        assert_eq!(eval_formula("00&"), false); // 1 AND 0 = false
    }

    #[test]
    fn test_or_operator() {
        assert_eq!(eval_formula("10|"), true); // 1 OR 0 = true
    }

    #[test]
    fn test_implication_operator() {
        assert_eq!(eval_formula("11>"), true); // 1 IMPLIES 1 = true
    }

    #[test]
    fn test_equivalence_operator() {
        assert_eq!(eval_formula("10="), false); // 1 EQUIV 0 = false
    }

    #[test]
    fn test_complex_formula() {
        assert_eq!(eval_formula("1011||="), true); // (1 OR (1 OR 0)) EQUIV 1 = true
    }

    /// these tests should produce an error
    #[test]
    fn test_invalid_formula_error() {
        let result = parse_rpn("1&");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Error: '&' operator requires 2 operands.");
    }
}

