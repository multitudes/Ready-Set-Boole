use ex08::powerset;

fn main() {
    println!("ex08 - Powerset --------------------------------\n");

    // Empty set
    println!("Powerset of []:");
    let result = powerset(vec![]);
    for subset in &result {
        println!("  {:?}", subset);
    }
    println!("Length: {}\n", result.len());

    // Single element
    println!("Powerset of [1]:");
    let result = powerset(vec![1]);
    for subset in &result {
        println!("  {:?}", subset);
    }
    println!("Length: {}\n", result.len());

    // Two elements
    println!("Powerset of [1, 2]:");
    let result = powerset(vec![1, 2]);
    for subset in &result {
        println!("  {:?}", subset);
    }
    println!("Length: {}\n", result.len());

    // Three elements
    println!("Powerset of [1, 2, 3]:");
    let result = powerset(vec![1, 2, 3]);
    for subset in &result {
        println!("  {:?}", subset);
    }
    println!("Length: {}\n", result.len());

    // Four elements
    println!("Powerset of [1, 2, 3, 4]:");
    let result = powerset(vec![1, 2, 3, 4]);
    for subset in &result {
        println!("  {:?}", subset);
    }
    println!("Length: {}\n", result.len());

    // With different values
    println!("Powerset of [10, 20, 30]:");
    let result = powerset(vec![10, 20, 30]);
    for subset in &result {
        println!("  {:?}", subset);
    }
    println!("Length: {}", result.len());
}
