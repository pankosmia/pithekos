// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs};
use std::path::Path;
use std::process::exit;
use pankosmia_web::rocket;
use tokio::runtime::Runtime;
use serde_json::json;
use rocket::fs::relative;
use std::io::Write;
use home::home_dir;

fn maybe_os_quoted_path_str(s: String) -> String {
    match env::consts::OS {
        "windows" => s.replace("\\", "\\\\").replace("/", "\\\\"),
        _ => s
    }
}
fn rocket1() -> () {
    let rt = Runtime::new().unwrap();
    let args: Vec<String> = env::args().collect();
    let mut working_dir = "".to_string();
    if args.len() == 2 {
        working_dir = args[1].clone();
    };
    let webfont_path = relative!("../webfonts");
    let app_setup_path = relative!("../setup/app_setup.json");
    let local_setup_path = relative!("../setup/local_setup.json");
    let app_resources_path = relative!("..");
    let local_setup_path_exists = Path::new(&local_setup_path).is_file();
    // Create local setup file if necessary (excluded from git)
    if !local_setup_path_exists {
        let stub_local_setup_content = json!({
            "local_pankosmia_path": format!("{}/{}",
                home_dir().unwrap().as_os_str().to_str().unwrap().to_string(),
                "repos/pankosmia"
            )
        });
        let mut file_handle = match fs::File::create(&local_setup_path) {
            Ok(h) => h,
            Err(e) => {
                println!("Could not open local setup file '{}' to write default: {}", local_setup_path, e);
                exit(1);
            }
        };
        let content_string = maybe_os_quoted_path_str(
            serde_json::to_string(&stub_local_setup_content).unwrap()
        );
        match file_handle.write_all(&content_string.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                println!("Could not write default local setup file to '{}: {}'", local_setup_path, e);
                exit(1);
            }
        }
    }
    let conf = json!({
        "working_dir": working_dir,
        "webfont_path": webfont_path,
        "local_setup_path": local_setup_path,
        "app_setup_path": app_setup_path,
        "app_resources_path": app_resources_path,
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
    pithekos_app_lib::run()
}
