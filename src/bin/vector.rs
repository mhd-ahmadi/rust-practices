#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name: name,
            contents: vec![],
        }
    }

    fn create_file(&mut self, name: String) {
        let file = File { name: name };
        self.contents.push(file);
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }
}

fn main() {
    let mut folder = Folder::new(String::from("New Folder"));
    folder.create_file(String::from("test.txt"));
    folder.create_file(String::from("sample.txt"));
    println!("{:#?}", folder);

    folder.delete_file(1);
    println!("{:#?}", folder);

    match folder.get_file(0) {
        Some(file) => println!("Retrieve file: {file:?}"),
        None => println!("There is no file!"),
    }
}


fn basic() {
    //let pizza = Vec::<i32>::new();
    //let pastas = vec!["Angle hair"];

    let mut pizza_diameters = vec![8, 10, 12, 14];
    pizza_diameters.push(16);
    pizza_diameters.push(18);
    pizza_diameters.insert(0, 6);

    let last_pizza_diameter = pizza_diameters.pop();
    println!("{:?}", last_pizza_diameter);

    let pizza_slice = &pizza_diameters[1..3];
    println!("{pizza_slice:?}");

    let peperoni = "Peporini".to_string();
    let mushroom = "Mushroom".to_string();
    let sausage = "Sausage".to_string();
    let pizza_toppings = vec![peperoni, mushroom, sausage];
    let value = &pizza_toppings[2];
    println!("{value}");
    
    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!("Length: {}, Capacity: {}", seasons.len(), seasons.capacity());
    seasons.push("Spring");
    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    println!("Length: {}, Capacity: {}", seasons.len(), seasons.capacity());
    seasons.push("New Season");
    println!("Length: {}, Capacity: {}", seasons.len(), seasons.capacity());
}