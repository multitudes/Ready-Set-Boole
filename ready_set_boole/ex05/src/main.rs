use ex05::negation_normal_form;

fn main() {
    println!("ex05 - Negation Normal Form --------------");   
    
    // De Morgan's laws - Negation of AND
    println!("\nDe Morgan's Law - AND:");
    println!("AB&! -> {}", negation_normal_form("AB&!"));
    // Expected: A!B!|
    
    // De Morgan's laws - Negation of OR
    println!("\nDe Morgan's Law - OR:");
    println!("AB|! -> {}", negation_normal_form("AB|!"));
    // Expected: A!B!&
    
    // Material condition elimination
    println!("\nMaterial Condition (A => B = !A | B):");
    println!("AB> -> {}", negation_normal_form("AB>"));
    // Expected: A!B|
    
    // Equivalence elimination
    println!("\nEquivalence (A <=> B):");
    println!("AB= -> {}", negation_normal_form("AB="));
    // Expected: AB&A!B!&|
    
    // Complex formula with negation of OR and AND
    println!("\nComplex - Negation of (A | (B & C)):");
    println!("AB|C&! -> {}", negation_normal_form("AB|C&!"));
    // Expected: A!B!&C!|
    
    // Double negation elimination
    println!("\nDouble Negation:");
    println!("A!! -> {}", negation_normal_form("A!!"));
    // Expected: A
    
    // Triple negation
    println!("A!!! -> {}", negation_normal_form("A!!!"));
    // Expected: A!
    
    // Complex with multiple operators
    println!("\nComplex - (A & B) => C:");
    println!("AB&C> -> {}", negation_normal_form("AB&C>"));
    // Expected: A!B!|C|
    
    // Negation of implication: !(A => B) = A & !B
    println!("\nNegation of Implication:");
    println!("AB>! -> {}", negation_normal_form("AB>!"));
    // Expected: AB!&
    
    // Negation of equivalence
    println!("\nNegation of Equivalence:");
    println!("AB=! -> {}", negation_normal_form("AB=!"));
    // Expected: AB!&A!B&|
    
    // Three variable formula
    println!("\nThree Variables:");
    println!("ABC&| -> {}", negation_normal_form("ABC&|"));
    // Expected: ABC&| (no change needed)
    
    println!("ABC&|! -> {}", negation_normal_form("ABC&|!"));
    // Expected: A!B!C!&| (De Morgan's applied)
    
    // Complex nested formula
    println!("\nComplex Nested:");
    println!("AB&C|! -> {}", negation_normal_form("AB&C|!"));
    // Expected: A!B!&C!&
    
    println!("AB|C&! -> {}", negation_normal_form("AB|C&!"));
    // Expected: A!B!&C!|

    // let formula = "AB&C|!"; // !( (A & B) | C )
    // if let Ok(tree) = parse_rpn(formula) {
    //     println!("Original Tree:");
    //     tree.print_tree();
    // }
}