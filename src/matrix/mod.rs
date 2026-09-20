// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    fmt::{self, Write},
    ops::{Add, AddAssign, Sub, SubAssign, Index, IndexMut}
};

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── Constants ───────────────────────────────────────────────────────────────
const EPSILON: f64 = 1e-10;

// ── `struct Matrix<N, M>` Definition ────────────────────────────────────────
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<const M: usize, const N: usize> {
    elements: [[f64; N]; M]
}

// ── `Matrix<N, M>` Implementation ───────────────────────────────────────────
impl<const M: usize, const N: usize> Matrix<M, N> {
    // ── Functions ───────────────────────────────────────────────────────────
    pub fn new(elements: [[f64; N]; M]) -> Self { Self { elements } }
    
    #[inline]
    pub const fn zeros() -> Self { Self { elements: [[0.0; N]; M] } }

    // ── Methods ─────────────────────────────────────────────────────────────
    fn clear_imprecis(&mut self) {
        for row in &mut self.elements {
            for val in row {
                if val.abs() < EPSILON {
                    *val = 0.0;
                }
            }
        }
    }
   
    pub fn rref(&mut self) {
        let mut lead = 0;

        for r in 0..M {
            if lead >= N {
                break;
            }

            // 1. Find a row with a non-zero element in column `lead`
            let mut i = r;
            while self.elements[i][lead].abs() < EPSILON {
                i += 1;
                if i == M {
                    i = r;
                    lead += 1;
                    if lead == N {
                        self.clear_imprecis();
                        return;
                    }
                }
            }

            // 2. Swap the current row `r` with row `i`
            self.elements.swap(i, r);

            // 3. Scale the row to make the pivot element equal to 1.0
            let pivot = self.elements[r][lead];
            for c in 0..N {
                self.elements[r][c] /= pivot;
            }

            // 4. Eliminate all other entries in column `lead`
            for row in 0..M {
                if row != r {
                    let factor = self.elements[row][lead];
                    for col in 0..N {
                        self.elements[row][col] -= factor * self.elements[r][col];
                    }
                }
            }

            lead += 1;
        }

        self.clear_imprecis();
    }

    pub fn as_rref(&self) -> Self {
        let mut self_clone = self.clone();
        self_clone.rref();
        self_clone
    }

    pub fn scale(&mut self, scaler: f64) {
        for i in 0..M {
            for j in 0..N {
                self.elements[i][j] *= scaler;
            }
        }
    }

    pub fn as_scaled(&self, scaler: f64) -> Self {
        let mut self_clone = self.clone();
        self_clone.scale(scaler);
        self_clone
    }
}

// ── `Display` Implementations ───────────────────────────────────────────────
impl<const M: usize, const N: usize> fmt::Display for Matrix<M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for (i, row) in self.elements.iter().enumerate() {
            if i > 0 {
                let _ = writeln!(buffer);
            }
            let _ = write!(buffer, "[");
            for (j, val) in row.iter().enumerate() {
                if j > 0 {
                    let _ = write!(buffer, "\t");
                }
                let _ = write!(buffer, "{val:.2}");
            }
            let _ = write!(buffer, "]");
        }
        f.write_str(&buffer)
    }
}

// ── `From` Implementations ──────────────────────────────────────────────────
impl<const M: usize, const N: usize> From<[[f64; N]; M]> for Matrix<M, N> {
    fn from(array: [[f64; N]; M]) -> Self { Self { elements: array } }
}

// ── `Index` Implementations ─────────────────────────────────────────────────
impl<const M: usize, const N: usize> Index<usize> for Matrix<M, N> {
    type Output = [f64; N];

    fn index(&self, i: usize) -> &Self::Output { &self.elements[i] }
}

impl<const M: usize, const N: usize> Index<(usize, usize)> for Matrix<M, N> {
    type Output = f64;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.elements[i][j]
    }
}

// ── `IndexMut` Implementations ──────────────────────────────────────────────
impl<const M: usize, const N: usize> IndexMut<usize> for Matrix<M, N> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.elements[i]
    }
}

impl<
    const M: usize,
    const N: usize
> IndexMut<(usize, usize)> for Matrix<M, N> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.elements[i][j]
    }
}


// ── `Add` Implementations ───────────────────────────────────────────────────
impl<const M: usize, const N: usize> Add for Matrix<M, N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = self.clone();
        
        for i in 0..M {
            for j in 0..N {
                sum.elements[i][j] += rhs.elements[i][j];
            }
        }

        sum
    }
}

// ── `AddAssign` Implementations ─────────────────────────────────────────────
impl<const M: usize, const N: usize> AddAssign for Matrix<M, N> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..M {
            for j in 0..N {
                self.elements[i][j] += rhs.elements[i][j];
            }
        }
    }
}

// ── `Sub` Implementations ───────────────────────────────────────────────────
impl<const M: usize, const N: usize> Sub for Matrix<M, N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut diff = self.clone();
        
        for i in 0..M {
            for j in 0..N {
                diff.elements[i][j] -= rhs.elements[i][j];
            }
        }

        diff
    }
}

// ── `SubAssign` Implementations ─────────────────────────────────────────────
impl<const M: usize, const N: usize> SubAssign for Matrix<M, N> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..M {
            for j in 0..N {
                self.elements[i][j] -= rhs.elements[i][j];
            }
        }
    }
}


