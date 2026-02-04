use ex06::conjunctive_normal_form;

fn main() {
    println!("ex06 - Conjunctive Normal Form --------------");

    // 1. Basic De Morgan (Already in CNF)
    println!(
        "De Morgan AND: AB&! -> {} (Expected: A!B!|)",
        conjunctive_normal_form("AB&!")
    );

    // 2. Simple Distributivity: A | (B & C)
    // Formula: A | (B & C) -> (A | B) & (A | C)
    println!(
        "Distribute OR over AND: ABC&| -> {} (Expected: AB|AC|&)",
        conjunctive_normal_form("ABC&|")
    );

    // 3. Right-side Distributivity: (A & B) | C
    // Formula: (A & B) | C -> (A | C) & (B | C)
    println!(
        "Distribute AND over OR: AB&C| -> {} (Expected: AC|BC|&)",
        conjunctive_normal_form("AB&C|")
    );

    // 4. Double Distributivity (The Explosion)
    // Formula: (A & B) | (C & D)
    // Expected: (A|C) & (A|D) & (B|C) & (B|D)
    // RPN: AC|AD|&BC|BD|&& (Order may vary depending on recursion)
    println!(
        "Complex Distributivity: AB&CD&| -> {}",
        conjunctive_normal_form("AB&CD&|")
    );

    // 5. Implication to CNF
    // Formula: A => (B & C) -> !A | (B & C) -> (!A | B) & (!A | C)
    println!(
        "Implication: ABC&> -> {} (Expected: A!B|A!C|&)",
        conjunctive_normal_form("ABC&>")
    );
}
