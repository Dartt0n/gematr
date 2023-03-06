#![allow(unused)]
mod core;

use rust_decimal_macros::dec;
use self::core::math::{Matrix};

fn main() {
    // todo: compile-time size checks
    // todo: determinant (require indexing with checks)
    // todo: inverse (require determinant or gauss elimination)
    // todo: solve algebra

    let a = matrix!(
        1, 2, 3;
        3, 4, 5;
    );

    let b = matrix!(
        3, 2, 1;
        5, 4, 3;
    );

    let c = matrix!(
        3;
        2;
        1;
    );

    let x = matrix!( a + b );
    let y = matrix!( a - b );
    let z = matrix!( a * c );

    let w = matrix!( a + b ); // value of a & b is not dropped

    let t = matrix!(a[0..1][0..1]); // inner-matrix
    let k = matrix!(a[0..2][0..2]);

    // dynamic indexing is still not possible...
}
