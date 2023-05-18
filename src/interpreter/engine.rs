use std::{str::FromStr, rc::Rc};

use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

use crate::analyzer::token;

pub fn evaluate(expression: token::Kind, arguments: Vec<Rc<Decimal>>) -> Rc<Decimal> {
    
    match expression {
        token::Kind::Number(number) => {
            return Rc::new(Decimal::from_str(&number).unwrap());
        }

        token::Kind::Func(name) => {
            let args: Vec<&Rc<Decimal>> = arguments.iter().rev().collect();

            match name.as_str() {
                "min" => {
                    let min_value = args.iter().map(|r| *r).min().unwrap();
                    println!("Evaluated: min({:?}) = {}", &args, min_value);
                    
                    return Rc::clone(min_value);
                },
                "max" => {
                    let max_value = args.iter().map(|r| *r).max().unwrap();
                    println!("Evaluated: min({:?}) = {}", &args, max_value);

                    return Rc::clone(max_value);
                }

                _ => {}
            }
        },

        token::Kind::BinaryOperator(op) => {
            let a = arguments[1].as_ref();
            let b = arguments[0].as_ref();

            match op {
                token::BinOps::Plus => {
                    println!("Evaluated: {} + {} = {}", a, b, a+b);
                    return Rc::new(a + b);
                },

                token::BinOps::Minus => {
                    println!("Evaluated: {} - {} = {}", a, b, a-b);
                    return Rc::new(a - b);
                },

                token::BinOps::Mult => {
                    println!("Evaluated: {} * {} = {}", a, b, a*b);
                    return Rc::new(a * b);
                },

                token::BinOps::Div => {
                    println!("Evaluated: {} / {} = {}", a, b, a/b);
                    return Rc::new(a / b);
                },

                token::BinOps::Mod => {
                    println!("Evaluated: {} % {} = {}", a, b, a%b);
                    return Rc::new(a % b);
                },

                token::BinOps::Pow => {
                    println!("Evaluated: {} ^ {} = {}", a, b, a.powd(b.clone()));
                    return Rc::new(a.powd(b.clone()));
                },
            }
        },


        _ => {}
    }
    
    
    return Rc::new(dec!(0));

    
}