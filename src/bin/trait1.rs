/// Doc: https://doc.rust-lang.org/book/ch10-02-traits.html

use std::{collections::HashMap, fmt::Display};

trait Accommodation {
    fn book(&mut self, name: &str, nights: u16);
}

trait Description {
    fn get_description(&self) -> String {
        format!("A wonderful place to stay.")
    }
}

#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u16>,
}

impl<T> Hotel<T> {
    fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u16) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {}

struct AirBnB {
    host: String,
    guests: Vec<(String, u16)>,
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, name: &str, nights: u16) {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s appartment", self.host)
    }
}

fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

fn mix_and_match(
    first: &mut (impl Accommodation + Description),
    second: &mut impl Accommodation,
    guest: &str,
) {
    first.book(guest, 1);
    first.get_description();
    second.book(guest, 1);
}

fn mix_and_match_v2<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    first.book(guest, 1);
    first.get_description();
    second.book(guest, 1);
}

fn choose_best_place_to_stay() -> impl Accommodation + Description {
    Hotel::new("The Luxe")
}

fn main() {
    let mut hotel1 = Hotel::new(String::from("The Luxe"));
    println!("{}", hotel1.summarize());
    //let mut hotel = choose_best_place_to_stay();
    hotel1.book("Piers", 8);
    book_for_one_night(&mut hotel1, "New Customer");
    println!("{:#?}", hotel1);

    let hotel2 = Hotel::new("The Golden Standard");
    println!("{}", hotel2.summarize());

    let hotel3 = Hotel::new(vec!["The Sweet Escape", "Chamran Hotel"]);
    //hotel3.summarize(); Does not implement display trait!

    let mut airbnb = AirBnB::new("Peter");
    println!("{}", airbnb.get_description());
    airbnb.book("Homa", 7);
    book_for_one_night(&mut airbnb, "Anna");
    println!("{:#?}", hotel1);

    mix_and_match(&mut hotel1, &mut airbnb, "Couple customer");
    mix_and_match_v2(&mut hotel1, &mut airbnb, "Couple customer");

    let stays: Vec<&dyn Description> = vec![&hotel1, &hotel2, &hotel3, &airbnb];
    println!("{}", stays[0].get_description());
    println!("{}", stays[3].get_description());

    let mut stays2: Vec<&mut dyn Accommodation> = vec![&mut hotel1, &mut airbnb];
    stays2[0].book("Amanda", 2);
    println!("{:#?}", hotel1);
}
