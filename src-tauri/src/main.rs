// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if autodarkmode_lib::cli::handle_cli_args(&args) {
        return;
    }

    autodarkmode_lib::run();
}
