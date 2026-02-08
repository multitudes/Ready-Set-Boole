pub fn map(x: u16, y: u16) -> f64 {
    let mut z: u32 = 0;

    for i in 0..16 {
        // x bits go to even slots (0, 2, 4...)
        let bit_x = (x as u32 >> i) & 1;
        z |= bit_x << (2 * i);

        // y bits go to odd slots (1, 3, 5...)
        let bit_y = (y as u32 >> i) & 1;
        z |= bit_y << (2 * i + 1);
    }
    z as f64 / u32::MAX as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin() {
        // (0, 0) should map to 0.0
        assert_eq!(map(0, 0), 0.0);
    }

    #[test]
    fn test_max_values() {
        // (u16::MAX, u16::MAX) should map to 1.0
        assert_eq!(map(u16::MAX, u16::MAX), 1.0);
    }

    #[test]
    fn test_x_axis() {
        // Points along x-axis (y=0)
        let z1 = map(1, 0);
        let z2 = map(2, 0);
        assert!(z1 < z2);
    }

    #[test]
    fn test_y_axis() {
        // Points along y-axis (x=0)
        let z1 = map(0, 1);
        let z2 = map(0, 2);
        assert!(z1 < z2);
    }

    #[test]
    fn test_different_points_produce_different_values() {
        let z1 = map(5, 10);
        let z2 = map(10, 5);
        let z3 = map(100, 200);
        assert_ne!(z1, z2);
        assert_ne!(z1, z3);
        assert_ne!(z2, z3);
    }

    #[test]
    fn test_output_range() {
        // All outputs should be in [0.0, 1.0]
        for x in [0, 100, 1000, u16::MAX] {
            for y in [0, 100, 1000, u16::MAX] {
                let z = map(x, y);
                assert!(z >= 0.0 && z <= 1.0);
            }
        }
    }

    #[test]
    fn test_interleaving_pattern() {
        // Test the bit interleaving (Z-order curve)
        // (1, 0) -> bits: y=1 (0001), x=0 (0000) -> z interleaved = 0b01 = 1
        let z = map(1, 0);
        assert!(z > 0.0);

        // (0, 1) -> bits: y=0 (0000), x=1 (0001) -> z interleaved = 0b10 = 2
        let z2 = map(0, 1);
        assert!(z2 > z);
    }
}
