use std::io::stdin;

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    fn unlock(self, proc: impl FnOnce() -> String) -> Option<String> {
        let user_password = proc();
        if user_password == self.password {
            Some(self.treasure)
        } else {
            None
        }
    }

    //fn unlock<F: FnOnce() -> String>(self, proc: F) -> Option<String> {
}

fn main() {
    let vault = Vault {
        treasure: String::from("Gold"),
        password: String::from("topsecret"),
    };

    let hack = || {
        println!("Please provide a password to check vault:");
        let mut user_input = String::new();
        stdin().read_line(&mut user_input);
        user_input.trim().to_string()
    };

    let extraction = vault.unlock(hack);
    println!("{:?}", extraction)
}
