

fn main() {
    // let dog = String::from("Watson");

    // {
    //     let my_pet = &dog;
    //     println!("{my_pet}");
    // }

    // println!("{dog}");

    // {
    //     let my_pet = &dog;
    //     println!("{my_pet}");
    // }

    // bar();

    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    let two_cities = {
        let cities_ref = &cities;
        select_first_two_elements(cities_ref);
    };
    println!("{:?}", two_cities);
    select_first_two_elements(&cities);
}

fn bar() {
    let mut data = vec!['a', 'b', 'c'];
    let slice = &mut data[..];
    data.push('d');
    data.push('e');
    println!("{:#?}", data)
}

fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
    let selected_items = &items[..2];
    println!("{:?}", selected_items);
    selected_items
}

// aim to prevent dangling refrences
fn my_awsome_function(value: &i32, value2: &str, value3: String) -> &'a i32 { 
    value
}