

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
    // Ex03 doesn't have variables but will be used in ex04
    let empty_context = [false; 26];

    match parse_rpn(formula) {
        Ok(tree) => eval_node(&tree, &empty_context),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}


/// Abstract Syntax Tree (AST) node for Boolean formulas.
///
/// Represents the structure of a parsed Boolean expression in tree form.
/// Each node is either a leaf (value/variable) or an operator with children.
///
/// # Variants
///
/// * `Value(bool)` - A boolean constant: `true` (1) or `false` (0)
/// * `Variable(char)` - A variable identifier ('A'-'Z'), used in ex04+
/// * `Not(Box<Node>)` - Logical NOT (¬): unary negation operator
/// * `And(Box<Node>, Box<Node>)` - Logical AND (∧): conjunction
/// * `Or(Box<Node>, Box<Node>)` - Logical OR (∨): disjunction
/// * `Xor(Box<Node>, Box<Node>)` - Logical XOR (⊕): exclusive disjunction
/// * `Imply(Box<Node>, Box<Node>)` - Material implication (⇒): if-then
/// * `Equiv(Box<Node>, Box<Node>)` - Logical equivalence (⇔): if-and-only-if
///
/// # Examples
///
/// ```
/// use ex03::Node;
///
/// // Represents "1 AND 0" as a tree
/// let tree = Node::And(
///     Box::new(Node::Value(true)),
///     Box::new(Node::Value(false))
/// );
/// ```
#[derive(Debug, Clone)]
pub enum Node {
    /// A boolean constant: true (1) or false (0)
    Value(bool),
    ///Will be used in later exercises: holds variable 'A', 'B', etc.
    Variable(char),
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


/// Parses an RPN formula string into an Abstract Syntax Tree (AST).
///
/// Converts a Reverse Polish Notation formula into a tree structure
/// where each operator becomes a node with its operands as children.
///
/// # Arguments
///
/// * `formula` - A string slice containing the RPN expression
///
/// # Returns
///
/// * `Ok(Node)` - The root node of the parsed AST
/// * `Err(String)` - An error message if the formula is malformed
///
/// # Algorithm
///
/// Uses a stack-based approach:
/// 1. Push operands (0, 1) or variables (A-Z) onto the stack
/// 2. For unary operators (!), pop one operand and create a node
/// 3. For binary operators (&, |, ^, >, =), pop two operands and create a node
/// 4. Push the resulting node back onto the stack
/// 5. Final stack should contain exactly one node (the root)
///
/// # Errors
///
/// Returns an error if:
/// - An operator has insufficient operands
/// - The formula contains invalid characters
/// - The final stack size is not exactly 1
///
/// # Examples
/// ```
/// use ex03::parse_rpn;
///
/// let result = parse_rpn("10&");
/// assert!(result.is_ok());
///
/// let error = parse_rpn("1&");
/// assert!(error.is_err());
/// ```
pub fn parse_rpn(formula: &str) -> Result<Node, String> {
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
            'A'..='Z' => stack.push(Node::Variable(c)),
            c if c.is_whitespace() => continue,
            _ => return Err(format!("Error: Invalid character '{}' in formula.", c)),
        }
    }

    if stack.len() != 1 {
        return Err(format!("Error: Invalid RPN sequence (stack size is {} at end).", stack.len()));
    }
    Ok(stack.pop().unwrap())
}

/// Recursively evaluates an AST node to a boolean value.
///
/// Traverses the Abstract Syntax Tree depth-first, evaluating each node
/// according to its operator type and returning the computed result.
///
/// # Arguments
///
/// * `node` - The AST node to evaluate
/// * `values` - An array of 26 boolean values for variables A-Z (index 0 = 'A', etc.)
///
/// # Returns
///
/// The boolean result of evaluating the node and its children
///
/// # Algorithm
///
/// - For `Value` nodes: returns the stored boolean
/// - For `Variable` nodes: looks up the value in the `values` array
/// - For operator nodes: recursively evaluates children and applies the operation
///
/// # Examples
/// ```
/// use ex03::{Node, eval_node};
///
/// let tree = Node::And(
///     Box::new(Node::Value(true)),
///     Box::new(Node::Value(false))
/// );
/// let values = [false; 26];
/// assert_eq!(eval_node(&tree, &values), false);
/// ```
pub fn eval_node(node: &Node, values: &[bool; 26]) -> bool {
    match node {
        Node::Value(b) => *b,
        Node::Variable(c) => values[(*c as usize) - ('A' as usize)],
        Node::Not(a) => !eval_node(a, values), 
        Node::And(a, b) => eval_node(a, values) & eval_node(b, values),
        Node::Or(a, b) => eval_node(a, values) | eval_node(b, values),
        Node::Xor(a, b) => eval_node(a, values) ^ eval_node(b, values),
        Node::Imply(a, b) => !eval_node(a, values) | eval_node(b, values),
        Node::Equiv(a, b) => eval_node(a, values) == eval_node(b, values),
    }
}


/// pretty printing
impl Node {
    /// Prints a visual tree representation of the AST to stdout.
    ///
    /// Displays the tree structure using box-drawing characters,
    /// making it easy to visualize the formula's hierarchy.
    ///
    /// # Examples
    /// ```
    /// use ex03::{Node, parse_rpn};
    ///
    /// if let Ok(tree) = parse_rpn("10&") {
    ///     tree.print_tree();
    /// }
    /// // Output:
    /// // &
    /// // ├──1
    /// // └──0
    /// ```
    pub fn print_tree(&self) {
        self.print_recursive("", true, true);
    }

    fn print_recursive(&self, prefix: &str, is_last: bool, is_root: bool) {
        // 1. Determine the symbols for this level
        let connector = if is_root {
            ""
        } else if is_last {
            "└──"
        } else {
            "├──"
        };

        // 2. Print the current node's label
        print!("{}", prefix);
        print!("{}", connector);
        match self {
            Node::Value(b) => println!("{}", if *b { '1' } else { '0' }),
            Node::Variable(c) => println!("{}", c),
            Node::Not(_) => println!("!"),
            Node::And(_, _) => println!("&"),
            Node::Or(_, _) => println!("|"),
            Node::Xor(_, _) => println!("^"),
            Node::Imply(_, _) => println!(">"),
            Node::Equiv(_, _) => println!("="),
        }

        // 3. Calculate the prefix for children
        let new_prefix = if is_root {
            String::new()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        // 4. Recurse through children
        match self {
            Node::Not(child) => {
                child.print_recursive(&new_prefix, true, false);
            }
            Node::And(l, r) | Node::Or(l, r) | Node::Xor(l, r) | 
            Node::Imply(l, r) | Node::Equiv(l, r) => {
                l.print_recursive(&new_prefix, false, false);
                r.print_recursive(&new_prefix, true, false);
            }
            _ => {} // Leaves (Value/Variable) have no children
        }
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

