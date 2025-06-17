use std::ops::Add;

#[derive(Debug)]
struct Launch {
    cost: f64,
}

impl Add for Launch {
    type Output = f64;

    fn add(self, rhs: Self) -> Self::Output {
        self.cost + rhs.cost
    }
}

fn main() {
    let one = Launch { cost: 19.99 };
    let two = Launch { cost: 29.99 };
    println!("{:.2}", one + two);
}
