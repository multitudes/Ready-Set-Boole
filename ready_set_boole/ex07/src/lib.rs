use ex03::{Node, parse_rpn};
use ex04::{generate_truth_table, get_variables};

/// Determines if a Boolean formula is satisfiable (SAT).
///
/// A formula is satisfiable if there exists at least one assignment of
/// variable values that makes the entire formula evaluate to `true`.
/// This is the classic Boolean Satisfiability Problem (SAT).
///
/// # Arguments
///
/// * `formula` - A string slice containing the Boolean formula in Reverse Polish Notation (RPN)
///
/// # Returns
///
/// `true` if the formula is satisfiable (at least one assignment makes it true)
/// `false` if the formula is unsatisfiable (all assignments make it false)
///
/// # SAT Classification
///
/// - **Satisfiable**: At least one row in the truth table outputs `true`
/// - **Unsatisfiable (Contradiction)**: All rows output `false` (e.g., `A & !A`)
/// - **Tautology**: All rows output `true` (always satisfiable)
///
/// # Algorithm
///
/// 1. Parse the RPN formula into an Abstract Syntax Tree (AST)
/// 2. Extract all variables from the formula
/// 3. Generate a complete truth table (all $2^n$ variable combinations)
/// 4. Check if any row evaluates to `true`
/// 5. Return `true` if found, `false` otherwise
///
/// # Time Complexity
///
/// O($2^n$) where $n$ is the number of variables (exponential)
///
/// # Panics
///
/// Panics if the formula is malformed (invalid RPN syntax)
///
/// # Examples
///
/// ```
/// use ex07::sat;
///
/// // Satisfiable formulas
/// assert!(sat("A"));                    // true when A=1
/// assert!(sat("AB|"));                  // true when A=1 or B=1
/// assert!(sat("AA!|"));                 // Tautology (always true)
///
/// // Unsatisfiable formulas
/// assert!(!sat("0"));                   // Always false
/// assert!(!sat("AA!&"));                // A & !A is always false
/// assert!(!sat("AA^"));                 // A XOR A is always false
///
/// // Complex formula
/// assert!(sat("ABC|&"));                // A & (B | C) - satisfiable
/// ```
pub fn sat(formula: &str) -> bool {
    let tree: Node = match parse_rpn(formula) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error parsing formula: {}", e);
            std::process::exit(1);
        }
    };
    let vars = get_variables(formula);
    let table = generate_truth_table(&tree, &vars);

    // If any row has output 'true', it's satisfiable
    table.rows.iter().any(|row| row.output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_sat() {
        assert!(sat("A"));
        assert!(sat("AB|"));
        assert!(sat("AB&"));
    }

    #[test]
    fn test_contradictions() {
        // These should never be true, no matter the input
        assert!(!sat("0"));
        assert!(!sat("AA!&"));
        assert!(!sat("AA^"));
        // (A > B) & (A & B!) -> (!A | B) & (A & !B) -> Contradiction
        assert!(!sat("AB>AB!&&"));
    }

    #[test]
    fn test_tautologies() {
        // These are always true, so they are definitely satisfiable
        assert!(sat("1"));
        assert!(sat("AA!|"));
        assert!(sat("AA="));
    }

    #[test]
    fn test_complex_formulas() {
        // "ABC|&" means A & (B | C)
        // Satisfiable if A=1 and (B=1 or C=1)
        assert!(sat("ABC|&"));

        // A complicated one: (A | B) & !A & !B
        assert!(!sat("AB|A!&B!&"));
    }
}
