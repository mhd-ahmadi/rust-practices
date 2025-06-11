enum LaundryCycle {
    Cold,
    Hot { temperature: u16 },
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => println!("Running with cold temperature"),
            LaundryCycle::Hot { temperature } => println!("Running with temperature of {}", temperature),
            LaundryCycle::Delicate(fabric_type) => println!("Running with a delicate cycle for {}", fabric_type),
        }
    }
}
fn main() {
    let cycle1 = LaundryCycle::Cold;
    let cycle2 = LaundryCycle::Hot { temperature: 79 };
    let cycle3 = LaundryCycle::Delicate(String::from("Silk"));

    cycle1.wash_laundry();
    cycle2.wash_laundry();
    cycle3.wash_laundry();
}