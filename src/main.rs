extern crate env_logger;
extern crate log;
extern crate notify;

use log::{debug, error, info};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use pathdiff::diff_paths;
use std::env;
use std::path::PathBuf;
use std::process;
use std::sync::mpsc::channel;
use std::time::Duration;

#[derive(Debug)]
struct Config {
    src: PathBuf,
    dest: PathBuf,
}

fn copy_to_dest(config: &Config, path: PathBuf) {
    info!("copying {} to {}", path.display(), config.dest.display());
    let relative_path = make_path_relative_to_src(&config, &path);
    debug!("{}", relative_path.display());
}

fn load_configuration_from_environment() -> Result<Config, String> {
    let src = match env::var("RETRO_SAVES") {
        Ok(val) => PathBuf::from(val),
        Err(e) => return Err(format!("RETRO_SAVES: {}", e)),
    };

    let dest = match env::var("RETRO_GAMES") {
        Ok(val) => PathBuf::from(val),
        Err(e) => return Err(format!("RETRO_GAMES: {}", e)),
    };

    return Ok(Config { src, dest });
}

fn main() {
    env_logger::init();

    let config = match load_configuration_from_environment() {
        Ok(config) => config,
        Err(e) => {
            error!("error: {}", e);
            process::exit(1)
        }
    };

    match validate_config(&config) {
        Ok(_) => (),
        Err(e) => {
            error!("error: {}", e);
            process::exit(1)
        }
    };

    debug!("{:?}", config);

    match watch(&config) {
        Ok(_) => debug!("process complete"),
        Err(e) => {
            error!("error: {}", e);
            process::exit(1);
        }
    }
}

fn make_path_relative_to_src(config: &Config, path: &PathBuf) -> PathBuf {
    return diff_paths(path, &config.src).expect("error: could not parse path");
}

fn validate_config(config: &Config) -> Result<(), String> {
    if !config.src.exists() {
        return Err(format!("source does not exist"));
    } else if !config.src.is_dir() {
        return Err(format!("source is not a directory"));
    }

    if !config.dest.exists() {
        return Err(format!("destination does not exist"));
    } else if !config.dest.is_dir() {
        return Err(format!("destination is not a directory"));
    }

    return Ok(());
}

fn watch(config: &Config) -> Result<(), String> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = match Watcher::new(tx, Duration::from_secs(2)) {
        Ok(val) => val,
        Err(e) => return Err(format!("{}", e)),
    };
    match watcher.watch(config.src.as_path(), RecursiveMode::Recursive) {
        Ok(_) => debug!("Watching {}", config.src.display()),
        Err(e) => return Err(format!("{}", e)),
    }

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Create(path) | DebouncedEvent::Write(path)) => {
                copy_to_dest(&config, path)
            }
            Ok(event) => debug!("skipping {:?}", event),
            Err(e) => error!("watch error: {}", e),
        }
    }
}
