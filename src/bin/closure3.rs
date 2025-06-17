fn main() {
    let mut game_console = String::from("PlayStation");

    let mut deleted_chrs = String::new();
    let closure = |chr| {
        let is_not_a = chr != 'a';
        if is_not_a {
            true
        } else {
            deleted_chrs.push(chr);
            false
        }
    };
    
    game_console.retain(closure);
    println!("{game_console}");
    println!("{deleted_chrs}");
}
