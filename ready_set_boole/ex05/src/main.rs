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
}