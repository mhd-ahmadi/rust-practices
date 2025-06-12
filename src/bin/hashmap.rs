use std::collections::{HashMap, HashSet};

fn main() {
    hashmap_sample();
    hashset_sample();
}

fn hashmap_sample() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat White", "Almond Milk");

    // change value if key is existed
    coffee_pairings.insert("Latte", "Pistachio Milk"); 

    // does allowed to insert if key not existed conditionally
    coffee_pairings.entry("Latte").or_insert("Pistacho Milk");

    //let ensure_value = coffee_pairings["Latte"];

    let value = coffee_pairings
        .get("Flat White")
        .copied() // due to ownership moved!
        .unwrap_or("Unknown Milk");

    println!("{}", value);
}

fn hashset_sample() {
    // for manage unique items
    let mut concert_queue: HashSet<&str> = HashSet::new();
    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    print!("{:?}", concert_queue);

    // this will reject insert because it already existed but with no error or panic!
    concert_queue.insert("Molly");

    println!("Remove Megan: {}", concert_queue.remove("Megan"));
    println!("Remove Megan: {}", concert_queue.remove("Franny"));

    println!("Existed Fred: {}", concert_queue.contains("Fred"));

    println!("{:?}", concert_queue.get("Molly"));

    let mut movie_queue: HashSet<&str> = HashSet::new();
    movie_queue.insert("Molly");

    println!("{:?}", concert_queue.union(&movie_queue));

    println!("{:?}", concert_queue.difference(&movie_queue));

    println!("{:?}", concert_queue.symmetric_difference(&movie_queue));

    println!("{:?}", concert_queue.is_disjoint(&movie_queue));

    // all values found in another type
    println!("{:?}", concert_queue.is_subset(&movie_queue));

    // 
    println!("{:?}", concert_queue.is_superset (&movie_queue));
}
