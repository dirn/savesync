use env_logger;
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
    bootstrap: bool,
}

fn bootstrap(config: &Config) {
    info!("bootstrapping");
    rsync(&config, PathBuf::from(""));
}

fn copy_to_dest(config: &Config, path: PathBuf) {
    if path.is_dir() {
        debug!("skipping {}", path.display());
        return;
    }

    let relative_path = make_path_relative_to_src(&config, &path);
    debug!("relative path: {}", relative_path.display());

    rsync(&config, relative_path);
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

    let maybe_bootstrap = match env::var("RETRO_BOOTSTRAP") {
        Ok(val) => val.to_lowercase(),
        Err(_) => "".to_string(),
    };
    // This list is intentionally a superset of what's documented.
    let bootstrap = vec![
        "1".to_string(),
        "t".into(),
        "true".into(),
        "y".into(),
        "yes".into(),
    ]
    .contains(&maybe_bootstrap);

    return Ok(Config {
        src,
        dest,
        bootstrap,
    });
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

    if config.bootstrap {
        bootstrap(&config);
    }

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

fn rsync(config: &Config, path: PathBuf) {
    info!("syncing {}", path.display());

    let mut cmd = process::Command::new("rsync");
    let output = cmd
        .arg("--archive")
        .arg("--update")
        .arg("--relative")
        .arg(&config.src.join(".").join(path))
        .arg(&config.dest)
        .output();

    match output {
        Ok(value) => debug!("{:?}", value),
        Err(e) => error!("{}", e),
    }
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
