extern crate config;
extern crate notify;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

use config::{Config, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Deserialize)]
pub struct Cert {
    pub cert: String,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub debug: bool,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub cert: Cert,
    pub database: Database,
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/Settings"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // Now that we're done, let's access our configuration
        //println!("debug: {:?}", s.get_bool("server.debug"));
        //println!("database: {:?}", s.get::<String>("database.path"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}

pub fn show() {
    let settings = Settings::new();

    // Print out our settings
    println!("{:?}", settings);
}

pub fn _watch() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch("./config/Settings.toml", RecursiveMode::NonRecursive)
        .unwrap();

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Write(_)) => {
                println!(" * Settings.toml written; refreshing configuration ...");
                show();
            }

            Err(e) => println!("watch error: {:?}", e),

            _ => {
                // Ignore event
            }
        }
    }
}
