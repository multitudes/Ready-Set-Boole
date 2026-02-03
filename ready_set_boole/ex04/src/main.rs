use ex04::print_truth_table;

fn main() {
    println!("ex04 - --------------");   
    print_truth_table("CA&");
    print_truth_table("CA&E|");
    // print_truth_table("CA&E");
    print_truth_table("AB!!");

    // let formula = "AB&C|!"; // !( (A & B) | C )
    // if let Ok(tree) = parse_rpn(formula) {
    //     println!("Original Tree:");
    //     tree.print_tree();
    // }
}