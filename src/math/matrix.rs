use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

pub struct Matrix<const N: usize, const M: usize>(pub [[Decimal; M]; N]);

#[macro_export]
macro_rules! matrix {
    ($A:tt[$rs:tt..$re:tt][$cs:tt..$ce:tt])=>{{
        const C: usize = $ce-$cs; const R: usize = $re-$rs;
        &Matrix::<R, C>::generate(|i, j| { $A.0[i+$rs][j+$cs] })
    }};
    ($A:tt$op:tt$B:tt)=>{ &($A$op$B) };
    ($($($i:expr),+;)+)=>{ &Matrix([$([$(dec!($i),)+],)+]) };
}

impl<const N: usize, const M: usize> Matrix<N, M> {
    pub fn generate<F>(func: F) -> Self
    where
        F: Fn(usize, usize) -> Decimal,
    {
        let mut value = [[dec!(0.0); M]; N];
        for i in 0..N {
            for j in 0..M {
                value[i][j] = func(i, j);
            }
        }
        Self(value)
    }

    pub fn transpose(&self) -> Matrix<M, N> {
        Matrix::generate(|i, j| self.0[j][i])
    }
}

impl<const N: usize> Matrix<N, N> {
    pub fn identity() -> Self {
        Self::generate(|i, j| if i == j { dec!(1) } else { dec!(0) })
    }

    pub fn determinant(self) -> f64 {
        todo!()
    }

    pub fn inverse(self) -> Matrix<N, N> {
        todo!()
    }
}

impl<'a, 'b, const N: usize, const M: usize> Add<&'b Matrix<N, M>> for &'a Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn add(self, rhs: &'b Matrix<N, M>) -> Self::Output {
        Self::Output::generate(|i, j| self.0[i][j] + rhs.0[i][j])
    }
}

impl<'a, 'b, const N: usize, const M: usize> Sub<&'b Matrix<N, M>> for &'a Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn sub(self, rhs: &'b Matrix<N, M>) -> Self::Output {
        Self::Output::generate(|i, j| self.0[i][j] - rhs.0[i][j])
    }
}

impl<const N: usize, const M: usize> AddAssign for Matrix<N, M> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            for j in 0..M {
                self.0[i][j] += rhs.0[i][j];
            }
        }
    }
}

impl<const N: usize, const M: usize> SubAssign for Matrix<N, M> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            for j in 0..M {
                self.0[i][j] -= rhs.0[i][j];
            }
        }
    }
}

impl<'a, 'b, const N: usize, const M: usize, const K: usize> Mul<&'b Matrix<M, K>> for &'a Matrix<N, M> {
    type Output = Matrix<N, K>;

    fn mul(self, rhs: &'b Matrix<M, K>) -> Self::Output {
        Matrix::<N, K>::generate(|i, k| (0..M).map(|j| self.0[i][j] * rhs.0[j][k]).sum())
    }
}

impl<const N: usize, const M: usize> PartialEq for Matrix<N, M> {
    fn eq(&self, rhs: &Self) -> bool {
        for i in 0..N {
            for j in 0..M {
                if self.0[i][j] != rhs.0[i][j] {
                    return false;
                }
            }
        }

        return true;
    }
}

impl<const N: usize, const M: usize> Debug for Matrix<N, M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix {}x{}\n", N, M)?;

        for i in 0..N {
            for j in 0..M - 1 {
                write!(f, "{} ", self.0[i][j])?;
            }
            write!(f, "{}\n", self.0[i][M - 1])?;
        }

        Ok(())
    }
}
