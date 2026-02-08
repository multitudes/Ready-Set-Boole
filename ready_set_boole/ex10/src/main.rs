use ex10::map;

fn main() {
    println!("ex10 - Space-Filling Curve (Z-Order)");
    println!("=====================================\n");

    // Test corner cases
    println!("Corner points:");
    println!("map(0, 0) = {}", map(0, 0));
    println!("map(u16::MAX, 0) = {}", map(u16::MAX, 0));
    println!("map(0, u16::MAX) = {}", map(0, u16::MAX));
    println!("map(u16::MAX, u16::MAX) = {}\n", map(u16::MAX, u16::MAX));

    // Test Z-order pattern in small 4×4 grid
    println!("Z-order pattern in 4×4 grid:");
    println!("(x, y) → z-value");
    println!("-----------------");
    for y in 0..4 {
        for x in 0..4 {
            let z = map(x, y);
            println!("({}, {}) → {:.10}", x, y, z);
        }
        println!();
    }

    // Show the Z-pattern visually
    println!("Visual Z-pattern (showing relative ordering):");
    println!("Grid positions:");
    for y in 0..4 {
        for x in 0..4 {
            print!("({},{}) ", x, y);
        }
        println!();
    }
    println!();

    println!("Z-order indices:");
    for y in 0..4 {
        for x in 0..4 {
            let z = map(x, y);
            let index = (z * u32::MAX as f64) as u32;
            print!("{:4} ", index);
        }
        println!();
    }
    println!();

    // Test some random points
    println!("Random sample points:");
    let test_points = [
        (10, 20),
        (100, 200),
        (1000, 2000),
        (12345, 54321),
        (32768, 32768),
    ];

    for (x, y) in test_points {
        let z = map(x, y);
        println!("map({:5}, {:5}) = {:.15}", x, y, z);
    }
    println!();

    // Demonstrate bijectivity (no collisions in sample)
    println!("Testing bijectivity (100×100 grid):");
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut collisions = 0;

    for x in 0..100 {
        for y in 0..100 {
            let z = map(x, y);
            let bits = z.to_bits();
            if seen.contains(&bits) {
                collisions += 1;
            }
            seen.insert(bits);
        }
    }

    println!("Points tested: 10,000");
    println!("Unique values: {}", seen.len());
    println!("Collisions: {}", collisions);
    println!("✓ Bijective: {}\n", collisions == 0);

    // Show locality comparison
    println!("Locality comparison:");
    let origin = map(1000, 1000);
    let nearby = map(1001, 1000);
    let far = map(5000, 5000);

    println!("Base point: (1000, 1000) → {:.15}", origin);
    println!("Nearby:     (1001, 1000) → {:.15}", nearby);
    println!("Far away:   (5000, 5000) → {:.15}", far);
    println!("Diff to nearby: {:.15}", (origin - nearby).abs());
    println!("Diff to far:    {:.15}\n", (origin - far).abs());

    println!("Demonstrating a Z-Curve 'Jump':");
    let a = map(127, 127);
    let b = map(128, 127); // Only 1 pixel away in 2D space!

    println!("Point (127, 127): {:.15}", a);
    println!("Point (128, 127): {:.15}", b);
    println!("Distance in 1D:   {:.15}", (a - b).abs());
    println!("Note: A small 1-pixel step in 2D caused a jump in 1D!");
}
