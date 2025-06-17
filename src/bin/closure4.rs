#[derive(Debug)]
struct Location {
    name: String,
    treasure: u32,
}

// The lifetime of the map struct will end before the lifetime of the locations referenced.
// We cannot have the map outlive the original data source, because if it does,
// then the location field will be a reference or a slice to some data that no longer exists.
// عمر داده‌هایی که Map به آن‌ها ارجاع دارد (locations) باید حداقل برابر یا بیشتر از عمر Map باشد.
struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location),
    {
        let final_index = self.locations.len() - 1;
        let mut current_index = 0;
        while current_index <= final_index {
            let current_location = &self.locations[current_index];
            action(current_location);
            current_index += 1;
        }
    }
}

fn main() {
    let locations = [
        Location {
            name: "Cave".into(),
            treasure: 100,
        },
        Location {
            name: "Forest".into(),
            treasure: 200,
        },
    ];
    let map = Map {
        locations: &locations,
    };
    let mut total_reasures = 0;
    map.explore(|loc| {
        total_reasures += loc.treasure
    });
    println!("Total treasures collected: {}", total_reasures);

    let mut location_names: Vec<String> = Vec::new();
    map.explore(|loc| location_names.push(loc.name.clone()));

    println!("{:?}", location_names)
}
