// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    fmt::{self, Write},
    ops::{
        Add,
        AddAssign,
        Mul,
        Sub,
        SubAssign,
        Index,
        IndexMut
    }
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

    #[inline]
    pub const fn is_square() -> bool { N == M }

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

    fn find_pivot(&self, r: usize, lead: &mut usize) -> Option<usize> {
        while *lead < N {
            if let Some(i) = (r..M).find(
                |&i| self.elements[i][*lead].abs() >= EPSILON
            ) {
                return Some(i);
            }
            *lead += 1;
        }
        None
    }
    
    #[inline]
    fn row_swap(&mut self, i: usize, j: usize) { self.elements.swap(i, j); }

    fn row_scale(&mut self, row_idx: usize, scaler: f64) {
        for col_idx in 0..N {
            self.elements[row_idx][col_idx] /= scaler;
        }
    }

    fn row_replace(&mut self, new_r: usize, c: usize) {
        for r in 0..M {
            if r != new_r {
                let factor = self.elements[r][c];
                for c in 0..N {
                    self.elements[r][c] -= factor * self.elements[new_r][c];
                }
            }
        }
    }
      
    pub fn rref(&mut self) {
        let mut lead = 0;

        for r in 0..M {
            let Some(i) = self.find_pivot(r, &mut lead) else { break };

            self.row_swap(i, r);
            self.row_scale(r, self.elements[r][lead]);
            self.row_replace(r, lead);

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

// ── `Mul` Implementations ───────────────────────────────────────────────────
impl<
    const M: usize,
    const N: usize,
    const P: usize
> Mul<&Matrix<N, P>> for &Matrix<M, N> {
    type Output = Matrix<M, P>;
    fn mul(self, rhs: &Matrix<N, P>) -> Self::Output {
        let mut product = [[0.0f64; P]; M];

        for i in 0..M {
            for k in 0..N {
                let a_ik = self.elements[i][k];
                for j in 0..P {
                    product[i][j] += a_ik * rhs.elements[k][j];
                }
            }
        }

        Matrix { elements: product }
    }
}

impl<
    const M: usize,
    const N: usize,
    const P: usize
> Mul<Matrix<N, P>> for &Matrix<M, N> {
    type Output = Matrix<M, P>;
    fn mul(self, rhs: Matrix<N, P>) -> Self::Output { self * &rhs }
}

impl<
    const M: usize,
    const N: usize,
    const P: usize
> Mul<&Matrix<N, P>> for Matrix<M, N> {
    type Output = Matrix<M, P>;
    fn mul(self, rhs: &Matrix<N, P>) -> Self::Output { &self * rhs }
}

impl<
    const M: usize,
    const N: usize,
    const P: usize
> Mul<Matrix<N, P>> for Matrix<M, N> {
    type Output = Matrix<M, P>;
    fn mul(self, rhs: Matrix<N, P>) -> Self::Output { &self * &rhs }
}
