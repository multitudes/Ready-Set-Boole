use ex05::negation_normal_form;

fn main() {
    println!("ex05 - --------------");   
    println!("{}", negation_normal_form("AB&!"));
    // A!B!|
    println!("{}", negation_normal_form("AB|!"));
    // A!B!&
    println!("{}", negation_normal_form("AB>"));
    // A!B|
    println!("{}", negation_normal_form("AB="));
    // AB&A!B!&|
    println!("{}", negation_normal_form("AB|C&!"));
    // A!B!&C!|

    // let formula = "AB&C|!"; // !( (A & B) | C )
    // if let Ok(tree) = parse_rpn(formula) {
    //     println!("Original Tree:");
    //     tree.print_tree();
    // }
}