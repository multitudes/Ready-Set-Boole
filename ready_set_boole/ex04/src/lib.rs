use std::collections::BTreeSet;
use ex03::parse_rpn;

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
    let n = vars.len();

    // header
    for v in &vars {
        print!("| {v} ");
    }
    println!("| = |");
    for v in &vars {
        print!("|---");
    }
    println!("|---|")

}


/// 
fn get_variables(formula: &str) -> Vec<char> {
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
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
