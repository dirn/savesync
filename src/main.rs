use std::env;

fn main() {
    match env::var("RETRO_GAMES") {
        Ok(val) => println!("RETRO_GAMES: {}", val),
        Err(e) => println!("RETRO_GAMES: {}", e),
    }
}
