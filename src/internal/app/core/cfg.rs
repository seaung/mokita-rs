use config::Config;
use std::{fs, sync::OnceLock};

static CFG: OnceLock<Config> = OnceLock::new();

pub fn init_config(filepath: &str) {
    let path = fs::canonicalize(filepath).unwrap_or_else(|e| panic!("{}", e));

    let cfg = Config::builder()
        .add_source(config::File::with_name(path.to_str().unwrap()))
        .build()
        .unwrap_or_else(|e| panic!("{}", e));

    let _ = CFG.set(cfg);
}

pub fn config() -> &'static Config {
    CFG.get().expect("config not initialized.")
}
