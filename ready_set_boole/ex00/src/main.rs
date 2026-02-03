use ex00::adder;

fn main() {
    println!("adder --------------");
    
    // Basic addition
    println!("1 + 2 = {}", adder(1, 2));           // 3
    println!("13 + 37 = {}", adder(13, 37));       // 50
    
    // Zero cases
    println!("0 + 0 = {}", adder(0, 0));           // 0
    println!("5 + 0 = {}", adder(5, 0));           // 5
    println!("0 + 10 = {}", adder(0, 10));         // 10
    
    // Larger numbers
    println!("100 + 200 = {}", adder(100, 200));   // 300
    println!("1000 + 5000 = {}", adder(1000, 5000)); // 6000
    
    // Powers of 2
    println!("8 + 4 = {}", adder(8, 4));           // 12
    println!("16 + 32 = {}", adder(16, 32));       // 48
    
    // Large u32 values
    println!("u32::MAX - 1 + 1 = {}", adder(u32::MAX - 1, 1)); // u32::MAX
    println!("1000000 + 2000000 = {}", adder(1000000, 2000000)); // 3000000
    
    // Commutative property
    println!("7 + 3 = {}", adder(7, 3));           // 10
    println!("3 + 7 = {}", adder(3, 7));           // 10
}