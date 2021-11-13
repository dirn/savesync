use std::env;
use std::process;

fn main() {
    match env::var("RETRO_SAVES") {
        Ok(val) => println!("RETRO_SAVES: {}", val),
        Err(e) => {
            eprintln!("RETRO_SAVES: {}", e);
            process::exit(1);
        }
    }

    match env::var("RETRO_GAMES") {
        Ok(val) => println!("RETRO_GAMES: {}", val),
        Err(e) => {
            eprintln!("RETRO_GAMES: {}", e);
            process::exit(1);
        }
    }
}
