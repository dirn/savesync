use std::env;

fn main() {
    match env::var("RETRO_SAVES") {
        Ok(val) => println!("RETRO_SAVES: {}", val),
        Err(e) => println!("RETRO_SAVES: {}", e),
    }

    match env::var("RETRO_GAMES") {
        Ok(val) => println!("RETRO_GAMES: {}", val),
        Err(e) => println!("RETRO_GAMES: {}", e),
    }
}
