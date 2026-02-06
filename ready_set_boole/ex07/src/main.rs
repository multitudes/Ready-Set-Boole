use ex07::sat;

fn main() {
    println!("ex07 - SAT (Satisfiability) --------------");

    // Basic operations
    println!("AB|  -> {} (Expected: true)", sat("AB|"));
    println!("AB&  -> {} (Expected: true)", sat("AB&"));

    // Contradictions (Always False)
    println!("AA!& -> {} (Expected: false)", sat("AA!&")); // A AND NOT A
    println!("AA^  -> {} (Expected: false)", sat("AA^")); // A XOR A
    println!("0    -> {} (Expected: false)", sat("0"));

    // Tautologies (Always True)
    println!("AA!| -> {} (Expected: true)", sat("AA!|")); // A OR NOT A
    println!("1    -> {} (Expected: true)", sat("1"));

    // Complex satisfiable
    println!("AB&C|D^ -> {} (Expected: true)", sat("AB&C|D^"));

    // error malformed formula
    println!("AB&C|D^ -> {} (Expected: true)", sat("AB&C|^"));
}
