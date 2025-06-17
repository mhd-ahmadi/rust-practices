fn main() {
    let multiplier = 5;

    let multiply_by = |value: i32| value * multiplier;
    
    println!("Multiply: {}", multiply_by(2));

    let product = |a: i32, b: i32| -> i32 {
        println!("Calculating product for you...");
        a * b
    };

    println!("Product: {}", product(3, 12));

    let mut numbers = vec![4, 8, 15, 16, 23];
    let mut add_number = || numbers.push(100);
    add_number();
    add_number();
    println!("{:?}", numbers);

    let first_name = String::from("Mohammad");
    let capture_string = || {
        let person = first_name;
        println!("{}", person);
    };
    capture_string();
    //capture_string(); //only once!!!!

    move_data();
}

fn move_data() {
    let first_name = String::from("Mohammad");
    let last_name = String::from("Ahmadi");
    let capture_string = move || {
        println!("{} {}", first_name, last_name);
    };
    capture_string();
    capture_string();
    capture_string();
}
