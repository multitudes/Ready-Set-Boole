use ex01::multiplier;

fn main() {
    println!("ex01 - multiplier --------------");
    
    // Basic multiplication
    println!("1 * 2 = {}", multiplier(1, 2));     // 2
    println!("3 * 4 = {}", multiplier(3, 4));     // 12
    println!("5 * 7 = {}", multiplier(5, 7));     // 35
    
    // Zero cases
    println!("0 * 37 = {}", multiplier(0, 37));   // 0
    println!("0 * 0 = {}", multiplier(0, 0));     // 0
    println!("15 * 0 = {}", multiplier(15, 0));   // 0
    
    // One cases
    println!("1 * 1 = {}", multiplier(1, 1));     // 1
    println!("1 * 100 = {}", multiplier(1, 100)); // 100
    println!("50 * 1 = {}", multiplier(50, 1));   // 50
    
    // Powers of 2
    println!("2 * 2 = {}", multiplier(2, 2));     // 4
    println!("8 * 4 = {}", multiplier(8, 4));     // 32
    println!("16 * 16 = {}", multiplier(16, 16)); // 256
    
    // Larger numbers
    println!("100 * 100 = {}", multiplier(100, 100));     // 10000
    println!("1000 * 2 = {}", multiplier(1000, 2));       // 2000
    println!("123 * 456 = {}", multiplier(123, 456));     // 56088
    
    // Commutative property
    println!("7 * 6 = {}", multiplier(7, 6));     // 42
    println!("6 * 7 = {}", multiplier(6, 7));     // 42
}