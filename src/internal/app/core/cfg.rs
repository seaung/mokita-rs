use config::Config;
use std::fs;
use std::sync::OnceLock;

static CFG: OnceLock<Config> = OnceLock::new();

pub fn init_cfg(path: &str) {}

pub fn get_cfg() {}
