// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use pankosmia_web::rocket;
use tokio::runtime::Runtime;
use serde_json::json;
use rocket::fs::relative;
fn rocket1() -> () {
    let rt = Runtime::new().unwrap();
    let args: Vec<String> = env::args().collect();
    let mut working_dir = "".to_string();
    if args.len() == 2 {
        working_dir = args[1].clone();
    };
    let webfont_path = relative!("../webfonts");
    let app_setup_path = relative!("../setup/app_setup.json");
    let conf = json!({
        "working_dir": working_dir,
        "webfont_path": webfont_path,
        "app_setup_path": app_setup_path
    });
    let builder = rocket(conf);
    rt.block_on(
        async move {
            let _ = builder.launch().await;
        }
    );
}
fn main() {
    std::thread::spawn(move || rocket1());
    pithekos_lib::run()
}
