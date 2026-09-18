// ── Aliases ─────────────────────────────────────────────────────────────────
use num_traits::float::Float;

// ── `Matrix<T, N, M>` Implementations ───────────────────────────────────────
pub struct Matrix<T: Float, const M: usize, const N: usize>([[T; N]; M]);


