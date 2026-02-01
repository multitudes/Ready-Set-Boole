use ex03::eval_formula;

fn main() {
    println!("ex03 - --------------");
    println!("{:?}", eval_formula("10&"));
    println!("{:?}", eval_formula("10|"));
    println!("{:?}", eval_formula("11>"));
    println!("{:?}", eval_formula("10="));
    println!("{:?}", eval_formula("1011||="));

    // now returns Err instead of panicking
    println!("{:?}", eval_formula("011||="));
}