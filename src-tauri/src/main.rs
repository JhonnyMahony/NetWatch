// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    netwatch_lib::run()
}
