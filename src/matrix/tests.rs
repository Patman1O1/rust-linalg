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

// ── Method Tests ────────────────────────────────────────────────────────────
#[test]
fn test_rref() {
    let mut m0 = Matrix::new(
        [
            [3.0, 5.0, 7.0, 9.0],
            [1.0, 3.0, 5.0, 7.0],
            [5.0, 7.0, 9.0, 1.0]
        ]
    );

    let m0_rref = Matrix::new(
        [
            [1.0, 0.0, -1.0, 0.0],
            [0.0, 1.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    );

    m0.rref();
    assert_eq!(m0_rref, m0);


    let mut m1 = Matrix::new(
        [
            [2.0, -3.0, 2.0, 1.0],
            [4.0, -8.0, 12.0, 1.0],
            [0.0, 1.0, -4.0, 8.0]
        ]
    );
    
    let m1_rref = Matrix::new(
        [
            [1.0, 0.0, -5.0, 0.0],
            [0.0, 1.0, -4.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    );

    m1.rref();
    assert_eq!(m1_rref, m1);


    let mut m2 = Matrix::new(
        [
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0]
        ]
    );

    let m2_rref = Matrix::new(
        [
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0]
        ]
    );

    m2.rref();
    assert_eq!(m2_rref, m2);


    let mut m3 = Matrix::new(
        [
            [1.0, -2.0, 1.0, 0.0],
            [0.0, 2.0, -8.0, 8.0],
            [5.0, 0.0, -5.0, 10.0]
        ]
    );

    let m3_rref = Matrix::new(
        [
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, -1.0]
        ]
    );
    m3.rref();
    assert_eq!(m3_rref, m3);


    let mut m4 = Matrix::new(
        [
            [3.0, 2.0, 3.0, -2.0, 1.0],
            [1.0, 1.0, 1.0, 0.0, 3.0],
            [1.0, 2.0, 1.0, -1.0, 2.0]
        ]
    );

    let m4_rref = Matrix::new(
        [
            [1.0, 0.0, 1.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 0.0, 2.0],
            [0.0, 0.0, 0.0, 1.0, 3.0]
        ]
    );

    m4.rref();
    assert_eq!(m4_rref, m4);
}
