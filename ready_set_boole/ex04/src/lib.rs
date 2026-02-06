use ex03::{Node, eval_node, parse_rpn};
use std::collections::BTreeSet;

/// Prints the truth table for a given Boolean formula in Reverse Polish Notation (RPN).
///
/// The formula can contain:
/// - Uppercase letters 'A' through 'Z' representing variables.
/// - '!' (NOT), '&' (AND), '|' (OR), '^' (XOR), '>' (IMPLY), '=' (EQUIVALENT).
///
/// # Arguments
///
/// * `formula` - A string slice containing the RPN expression with variables.
///
/// # Output
///
/// Prints to standard output a formatted table showing all $2^n$ combinations
/// of the $n$ variables found in the formula, along with the resulting evaluation.
///
/// # Processing Logic
/// 1. **Variable Identification**: Parses the string to find all unique variables.
/// 2. **Combination Generation**: Iterates through all possible boolean states (0 or 1) for these variables.
/// 3. **AST Evaluation**: For each combination, evaluate the formula tree by substituting variables with their current state.
/// 4. **Formatting**: Prints a header (e.g., `| A | B | = |`) followed by the state rows.
///
/// # Examples
/// ```
/// // For formula "AB&" (A AND B)
/// // Output:
/// // | A | B | |
/// // |---|---|---|
/// // | 0 | 0 | 0 |
/// // | 0 | 1 | 0 |
/// // | 1 | 0 | 0 |
/// // | 1 | 1 | 1 |
/// use ex04::print_truth_table;
/// print_truth_table("AB&");
/// ```
pub fn print_truth_table(formula: &str) {
    let tree = match parse_rpn(formula) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error parsing formula: {}", e);
            std::process::exit(1);
        }
    };

    let vars = get_variables(formula);
    let table = generate_truth_table(&tree, &vars);

    // Print header
    println!();
    for v in &table.variables {
        print!("| {v} ");
    }
    println!("| = |");
    for _ in &table.variables {
        print!("|---");
    }
    println!("|---|");

    for row in &table.rows {
        for &val in &row.inputs {
            print!("| {} ", if val { 1 } else { 0 });
        }
        println!("| {} |", if row.output { 1 } else { 0 });
    }
}

/// Represents a complete truth table for a Boolean formula.
///
/// A truth table contains all possible variable combinations and their
/// corresponding formula evaluations. It's used to verify formula correctness
/// and as input for optimization techniques like Karnaugh maps.
///
/// # Fields
///
/// * `variables` - Ordered list of variable characters (e.g., `['A', 'B', 'C']`)
/// * `rows` - Vector of `TruthRow` containing inputs and outputs for each combination
///
/// # Examples
///
/// ```
/// use ex04::{TruthTable, TruthRow};
///
/// let table = TruthTable {
///     variables: vec!['A', 'B'],
///     rows: vec![
///         TruthRow { inputs: vec![false, false], output: false },
///         TruthRow { inputs: vec![false, true], output: false },
///         TruthRow { inputs: vec![true, false], output: false },
///         TruthRow { inputs: vec![true, true], output: true },
///     ],
/// };
/// assert_eq!(table.rows.len(), 4);
/// ```
#[derive(Debug, Clone)]
pub struct TruthTable {
    pub variables: Vec<char>,
    pub rows: Vec<TruthRow>,
}

/// Represents a single row in a truth table.
///
/// Each row corresponds to one complete assignment of variable values
/// and the resulting output of the formula.
///
/// # Fields
///
/// * `inputs` - Vector of boolean values for each variable in order
/// * `output` - The result of evaluating the formula with these inputs
///
/// # Examples
///
/// ```
/// use ex04::TruthRow;
///
/// let row = TruthRow {
///     inputs: vec![true, false],  // A=1, B=0
///     output: false,               // Result of formula
/// };
/// assert_eq!(row.inputs.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct TruthRow {
    pub inputs: Vec<bool>,
    pub output: bool,
}

/// Generates a truth table for a given AST.
///
/// Takes an Abstract Syntax Tree (AST) representing a Boolean formula
/// and a list of variables, then generates all possible combinations
/// of variable assignments and evaluates the formula for each.
///
/// # Arguments
///
/// * `tree` - A reference to the AST node representing the formula
/// * `variables` - A slice of variable characters (e.g., `['A', 'B', 'C']`)
///
/// # Returns
///
/// A `TruthTable` struct containing:
/// - `variables`: The ordered list of variables
/// - `rows`: A vector of `TruthRow` with inputs and outputs
///
/// # Processing Logic
///
/// 1. **Combination Generation**: Iterates through all $2^n$ combinations (where $n$ = number of variables)
/// 2. **Variable Assignment**: For each combination, assigns boolean values to variables
/// 3. **AST Evaluation**: Evaluates the formula tree with the current variable assignment
/// 4. **Row Collection**: Stores the input combination and output result
///
/// # Examples
///
/// ```
/// use ex04::generate_truth_table;
/// use ex03::parse_rpn;
///
/// let tree = parse_rpn("AB&").unwrap();
/// let table = generate_truth_table(&tree, &['A', 'B']);
/// assert_eq!(table.rows.len(), 4);  // 2^2 = 4 rows
/// assert_eq!(table.rows[3].output, true);  // A=1, B=1 -> 1 & 1 = true
/// ```
pub fn generate_truth_table(tree: &Node, variables: &[char]) -> TruthTable {
    let var_count = variables.len();
    let mut rows = Vec::new();

    // Generate all 2^n combinations
    for row in 0..(1 << var_count) {
        let mut values = [false; 26];
        let mut inputs = Vec::new();

        // Set variable values for this row
        for (idx, &v) in variables.iter().enumerate() {
            let var_value = (row >> (var_count - 1 - idx)) & 1 == 1;
            values[(v as usize) - ('A' as usize)] = var_value;
            inputs.push(var_value);
        }

        // Evaluate the formula
        let output = eval_node(tree, &values);
        rows.push(TruthRow { inputs, output });
    }

    TruthTable {
        variables: variables.to_vec(),
        rows,
    }
}

/// Extracts all variables from a Boolean formula in RPN.
///
/// Scans the formula string and identifies all uppercase letters
/// representing variables. Variables are sorted alphabetically
/// and deduplicated.
///
/// # Arguments
///
/// * `formula` - A string slice containing the RPN expression
///
/// # Returns
///
/// A vector of unique variable characters sorted in ascending order
///
/// # Examples
///
/// ```
/// use ex04::get_variables;
///
/// assert_eq!(get_variables("AB&"), vec!['A', 'B']);
/// assert_eq!(get_variables("ABC&|"), vec!['A', 'B', 'C']);
/// assert_eq!(get_variables("ABA&"), vec!['A', 'B']);  // Deduplication
/// assert_eq!(get_variables("10&"), vec![]);  // Constants only
/// ```
pub fn get_variables(formula: &str) -> Vec<char> {
    let mut set = BTreeSet::new();

    for c in formula.chars() {
        if c.is_ascii_uppercase() {
            set.insert(c);
        }
    }
    set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_variables() {
        assert_eq!(get_variables("AB&"), vec!['A', 'B']);
        assert_eq!(get_variables("ABA|&"), vec!['A', 'B']); // Deduplication
        assert_eq!(get_variables("ABC!&|"), vec!['A', 'B', 'C']); // Sorting
        assert_eq!(get_variables("10&"), vec![]); // No variables
    }

    #[test]
    fn test_get_variables_all_alphabet() {
        // Test with all 26 uppercase letters
        let formula = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let vars = get_variables(formula);
        assert_eq!(vars.len(), 26);
        assert_eq!(vars[0], 'A');
        assert_eq!(vars[25], 'Z');
    }

    #[test]
    fn test_get_variables_lowercase_ignored() {
        // Lowercase letters should be ignored
        assert_eq!(get_variables("ab&"), vec![]);
        assert_eq!(get_variables("Aa&"), vec!['A']);
        assert_eq!(get_variables("aAbBcC&"), vec!['A', 'B', 'C']);
    }

    #[test]
    fn test_get_variables_single_variable() {
        assert_eq!(get_variables("A!"), vec!['A']);
        assert_eq!(get_variables("Z"), vec!['Z']);
    }

    #[test]
    fn test_get_variables_duplicates() {
        assert_eq!(get_variables("AAAA&"), vec!['A']);
        assert_eq!(get_variables("ABABAB&"), vec!['A', 'B']);
    }

    #[test]
    fn test_variable_evaluation() {
        let tree = parse_rpn("AB&").unwrap();
        let mut values = [false; 26];

        // Case: A=1, B=0 -> 1 & 0 = false
        values[0] = true; // 'A'
        values[1] = false; // 'B'
        assert_eq!(eval_node(&tree, &values), false);

        // Case: A=1, B=1 -> 1 & 1 = true
        values[1] = true; // 'B'
        assert_eq!(eval_node(&tree, &values), true);
    }

    #[test]
    fn test_variable_evaluation_three_vars() {
        let tree = parse_rpn("ABC&|").unwrap();
        let mut values = [false; 26];

        // A=1, B=0, C=1 -> (1 & 0) | 1 = 0 | 1 = 1
        values[0] = true;
        values[1] = false;
        values[2] = true;
        assert_eq!(eval_node(&tree, &values), true);
    }

    #[test]
    fn test_variable_evaluation_xor() {
        let tree = parse_rpn("AB^").unwrap();
        let mut values = [false; 26];

        // A=1, B=0 -> 1 ^ 0 = 1
        values[0] = true;
        values[1] = false;
        assert_eq!(eval_node(&tree, &values), true);

        // A=1, B=1 -> 1 ^ 1 = 0
        values[1] = true;
        assert_eq!(eval_node(&tree, &values), false);
    }

    #[test]
    fn test_variable_evaluation_implication() {
        let tree = parse_rpn("AB>").unwrap();
        let mut values = [false; 26];

        // A=1, B=0 -> 1 > 0 = 0 (false implies false)
        values[0] = true;
        values[1] = false;
        assert_eq!(eval_node(&tree, &values), false);

        // A=0, B=1 -> 0 > 1 = 1 (true implies true)
        values[0] = false;
        values[1] = true;
        assert_eq!(eval_node(&tree, &values), true);
    }

    #[test]
    fn test_variable_evaluation_equivalence() {
        let tree = parse_rpn("AB=").unwrap();
        let mut values = [false; 26];

        // A=1, B=1 -> 1 = 1 = 1
        values[0] = true;
        values[1] = true;
        assert_eq!(eval_node(&tree, &values), true);

        // A=1, B=0 -> 1 = 0 = 0
        values[1] = false;
        assert_eq!(eval_node(&tree, &values), false);
    }

    #[test]
    fn test_invalid_rpn_handling() {
        let result = parse_rpn("A&"); // Missing operand
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_rpn_too_many_operands() {
        let result = parse_rpn("AB"); // No operator
        assert!(result.is_err());
    }

    #[test]
    fn test_mixed_variables_and_constants() {
        let tree = parse_rpn("1A&").unwrap();
        let mut values = [false; 26];

        // 1 & A=0 = 0
        values[0] = false;
        assert_eq!(eval_node(&tree, &values), false);

        // 1 & A=1 = 1
        values[0] = true;
        assert_eq!(eval_node(&tree, &values), true);
    }

    #[test]
    fn test_complex_formula_with_multiple_vars() {
        let tree = parse_rpn("AB&C|").unwrap();
        let mut values = [false; 26];

        // (A & B) | C
        // A=0, B=0, C=0 -> (0 & 0) | 0 = 0
        assert_eq!(eval_node(&tree, &values), false);

        // A=1, B=1, C=0 -> (1 & 1) | 0 = 1
        values[0] = true;
        values[1] = true;
        assert_eq!(eval_node(&tree, &values), true);

        // A=1, B=0, C=1 -> (1 & 0) | 1 = 1
        values[1] = false;
        values[2] = true;
        assert_eq!(eval_node(&tree, &values), true);
    }
}
