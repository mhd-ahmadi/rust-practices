#[derive(Debug)]
struct TaylorSwiftPopSong {
    title: String,
    release_year: u32,
    duration_seconds: u32,
}

impl TaylorSwiftPopSong {
    fn new(title: String, release_year: u32, duration_seconds: u32) -> Self {
        Self {
            title,
            release_year,
            duration_seconds,
        }
    }

    fn display_song_info(&self) {
        println!("Title is {}", self.title);
        println!("Release date is {}", self.release_year);
        println!("Year since release {}", self.years_since_release());
        println!("Duration seconds is {}", self.duration_seconds);
    }

    fn double_length(&mut self) {
        self.duration_seconds = self.duration_seconds * 2;
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_seconds > other.duration_seconds
    }

    fn years_since_release(&self) -> u32 {
        2024 - self.release_year
    }
}

fn main() {
    let mut blank_space = TaylorSwiftPopSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_seconds: 2 * 60 * 60,
    };

    let all_too_well = TaylorSwiftPopSong::new(
        String::from("All Too Well"), 2016, 13212378,
    );

    blank_space.display_song_info();
    blank_space.double_length();
    
    if blank_space.is_longer_than(&all_too_well) {
        println!("`{}` is longer than `{}`", blank_space.title, all_too_well.title)
    } else {
        println!("`{}` is shorter than `{}`", blank_space.title, all_too_well.title)
    }
}
