use ex10::map;
use ex11::reverse_map;

fn main() {
    println!("ex011 - Inverse Mapping Demo");
    println!("============================\n");

    // Sample points
    let points = [
        (0u16, 0u16),
        (1, 2),
        (100, 200),
        (12345, 54321),
        (u16::MAX, u16::MAX),
    ];

    println!("Forward then reverse (f⁻¹ ∘ f):");
    for (x, y) in points {
        let z = map(x, y);
        let (rx, ry) = reverse_map(z);
        println!(
            "map({:5}, {:5}) = {} -> reverse_map = ({:5}, {:5})",
            x, y, z, rx, ry
        );
    }

    println!("\nReverse then forward (f ∘ f⁻¹):");
    for (x, y) in points {
        let v = map(x, y);
        let (rx, ry) = reverse_map(v);
        let v2 = map(rx, ry);
        println!("reverse_map({}) = ({:5}, {:5}) -> map = {}", v, rx, ry, v2);
    }
}
