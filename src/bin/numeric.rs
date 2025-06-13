use std::{ops::Add, str::FromStr};

fn main() {
    let a: i32 = 5;
    let b: i32 = 10;
    let sum = a.add(b);
    println!("sum is: {sum}");

    let numeric_count = u64::from_str("5");
    println!("{}", numeric_count.unwrap_or(0))
}