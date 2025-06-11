#[derive(Debug)]
struct Food {
    name: String
}

#[derive(Debug)]
struct Resturant {
    reservations: u32,
    has_mice_infestation: bool
}

impl Resturant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return None;
        }
        if self.reservations > 12 {
            Some(Food { name: String::from("Uni Sushi") })
        }
        else {
            Some(Food { name: String::from("Strip Steak")})
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err(String::from("Sorry, we have a mice problem!"));
        }
        if address.is_empty() {
            return Err(String::from("No delivery address specified"));
        }
        Ok(Food { name: String::from("Burger") })
    }
}

fn main() {
    let marios = Resturant {
        reservations: 11,
        has_mice_infestation: true,
    };
    println!("{:?}", marios.chef_special());
    println!("{:?}", marios.deliver_burger("Qom, Pardisan"));
}