struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
    
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

fn main() {
    let a = Flight::new("New York", "London", "12:12");
    let b = Flight::new("New York", "London", "15:22");
    let c = Flight::new("New York", "Los Angeles", "23:04");

    println!("Flight A is Equal B? {}", a == b);
    println!("Flight A is Equal B? {}", a.eq(&b));
    println!("Flight B is Equal B? {}", b == c);
    println!("Flight B is Not Equal B? {}", b.ne(&c));
}
