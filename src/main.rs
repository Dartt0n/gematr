mod core;

use rust_decimal_macros::dec;
use self::core::math::{Matrix};

fn main() {
    // todo: implement operations using macros
    // todo: compile-time size checks
    // todo: determinant (require indexing with checks)
    // todo: inverse (require determinant or gauss elimination)
    // todo: solve algebra

    let x = matrix!
    (
        1;
        2;
        3;
        5;
        6;
    );

    let y = matrix!
    (
             1,   2,   3, -4.5,   5;
           0.0,   4, 3.6,    2,   1;
             1, 2.1,   3,    4,   5;
        1.3333,   4,   3,    2, 1.0;
    );

    let t = &y * &x;

    dbg!(t);
}
