use ex03::eval_formula;
use ex03::parse_rpn;

fn main() {
    println!("ex03 - --------------");
    println!("{:?}", eval_formula("10&"));
    println!("{:?}", eval_formula("10|"));
    println!("{:?}", eval_formula("11>"));
    println!("{:?}", eval_formula("10="));
    println!("{:?}", eval_formula("1011||="));
    println!("{:?}", eval_formula("1!!"));

    // try to pretty print
    let formula = "01&1|!";
    if let Ok(tree) = parse_rpn(formula) {
        println!("Original Tree:");
        tree.print_tree();
    }

    // now returns Err instead of panicking
    println!("{:?}", eval_formula("011||="));
}