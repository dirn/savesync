use std::env;
use std::process;

struct Config {
    src: String,
    dest: String,
}

fn load_configuration_from_environment() -> Result<Config, String> {
    let src = match env::var("RETRO_SAVES") {
        Ok(val) => val,
        Err(e) => return Err(format!("RETRO_SAVES: {}", e)),
    };

    let dest = match env::var("RETRO_GAMES") {
        Ok(val) => val,
        Err(e) => return Err(format!("RETRO_GAMES: {}", e)),
    };

    return Ok(Config { src, dest });
}

fn main() {
    let config = match load_configuration_from_environment() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1)
        }
    };
    println!("source: {}", config.src);
    println!("destination: {}", config.dest);
}
