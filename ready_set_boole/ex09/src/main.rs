use ex09::eval_set;

fn main() {
    println!("--- ex09: Set Evaluation ---");

    // 1. COMPLEMENT (!)
    // Demonstrates that NOT A depends on everything else in the sets
    let sets1 = vec![vec![1, 2], vec![3, 4]];
    println!("\n1. Complement Test");
    println!("Sets: A={:?}, B={:?}", sets1[0], sets1[1]);
    println!("A!   -> {:?} (Expected: [3, 4])", eval_set("A!", sets1));

    // 2. SYMMETRIC DIFFERENCE (^)
    // Elements in one or the other, but not both
    let sets2 = vec![vec![1, 2, 3], vec![3, 4, 5]];
    println!("\n2. XOR Test");
    println!("Sets: A={:?}, B={:?}", sets2[0], sets2[1]);
    println!(
        "AB^  -> {:?} (Expected: [1, 2, 4, 5])",
        eval_set("AB^", sets2)
    );

    // 3. MATERIAL IMPLICATION (>)
    // "If it's in A, it must be in B" (Vacuously true for elements not in A)
    let sets3 = vec![vec![1, 2], vec![2, 3]];
    println!("\n3. Implication Test");
    println!("Sets: A={:?}, B={:?}", sets3[0], sets3[1]);
    println!("AB>  -> {:?} (Expected: [2, 3])", eval_set("AB>", sets3));

    // 4. EQUIVALENCE (=) vs INTERSECTION (&)
    // This example proves they are different when the Universe contains "unclaimed" elements.
    let sets4 = vec![
        vec![1, 2], // A
        vec![1, 3], // B
        vec![4],    // C (This puts '4' into the Universe)
    ];
    println!("\n4. Equivalence vs Intersection");
    println!("Sets: A={:?}, B={:?}, C={:?}", sets4[0], sets4[1], sets4[2]);
    println!("Universe: {{1, 2, 3, 4}}");

    // Intersection only cares about what is SHARED
    println!(
        "AB&  -> {:?} (Expected: [1])",
        eval_set("AB&", sets4.clone())
    );

    // Equivalence cares about what is SHARED + what is SHARED ABSENCE
    // Elements in both: {1}
    // Elements in neither: {4} (Because 4 is in the universe but not in A or B)
    println!("AB=  -> {:?} (Expected: [1, 4])", eval_set("AB=", sets4));

    // 5. COMPLEX FORMULA
    let sets5 = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
    println!("\n5. Complex Formula (A & B) | C");
    println!("Sets: A={:?}, B={:?}, C={:?}", sets5[0], sets5[1], sets5[2]);
    println!(
        "AB&C| -> {:?} (Expected: [2, 3, 4])",
        eval_set("AB&C|", sets5)
    );
}
