// ── Aliases ─────────────────────────────────────────────────────────────────
use super::Matrix;

// ── Function Tests ──────────────────────────────────────────────────────────
#[test]
fn test_new() {
    let m = Matrix::new(
        [
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0]
        ]
    );

    // Row 1 
    assert_eq!(1.0, m[0][0]);
    assert_eq!(2.0, m[0][1]);
    assert_eq!(3.0, m[0][2]);

    // Row 2
    assert_eq!(4.0, m[1][0]);
    assert_eq!(5.0, m[1][1]);
    assert_eq!(6.0, m[1][2]);

    // Row 3
    assert_eq!(7.0, m[2][0]);
    assert_eq!(8.0, m[2][1]);
    assert_eq!(9.0, m[2][2]);
}

#[test]
fn test_zeros() {
    let m = Matrix::<3, 3>::zeros();

     // Row 1 
    assert_eq!(0.0, m[0][0]);
    assert_eq!(0.0, m[0][1]);
    assert_eq!(0.0, m[0][2]);

    // Row 2
    assert_eq!(0.0, m[1][0]);
    assert_eq!(0.0, m[1][1]);
    assert_eq!(0.0, m[1][2]);

    // Row 3
    assert_eq!(0.0, m[2][0]);
    assert_eq!(0.0, m[2][1]);
    assert_eq!(0.0, m[2][2]);

}

// ── Method Tests`` ──────────────────────────────────────────────────────────

