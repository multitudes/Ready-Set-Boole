use ex04::print_truth_table;
use ex03::parse_rpn;

fn main() {
    println!("ex04 - Truth Table Generator --------------");   
    
    // Two variables
    println!("\nFormula: CA&");
    print_truth_table("CA&");
    
    println!("\nFormula: CA&E|");
    print_truth_table("CA&E|");
    
    // Double negation
    println!("\nFormula: AB!!|");
    print_truth_table("AB!!|");
    
    // Single variable
    println!("\nFormula: A");
    print_truth_table("A");
    
    // All variables (A-Z)
    println!("\nFormula: AB&C|D^E>F=");
    print_truth_table("AB&C|D^E>F=");
    
    // Complex formula with multiple operations
    println!("\nFormula: ABC&|");
    print_truth_table("ABC&|");
    
    println!("\nFormula: AB&C|!");
    print_truth_table("AB&C|!");
    
    // De Morgan's laws
    println!("\nFormula: AB&!");
    print_truth_table("AB&!");
    
    println!("\nFormula: A!B!|");
    print_truth_table("A!B!|");

    println!("\npretty print: AB&C|!");
    let formula = "AB&C|!"; // !( (A & B) | C )
    if let Ok(tree) = parse_rpn(formula) {
        println!("Original Tree:");
        tree.print_tree();
    }
}