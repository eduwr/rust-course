use std::{env, fs::File};

pub fn get_user_folder() -> Option<String> {
    if let Some(home_path) = env::var_os("HOME") {
        Some(home_path.into_string().unwrap())
    } else {
        None
    }
}

pub fn create(path: &str, file_name: &str) {
    println!("path: {}", path);
    println!("file_name: {}", file_name);

    let file_path = format!(r"{}/{}", path, file_name);
    println!("file_path: {}", file_path);

    match File::create(&file_path) {
        Ok(_) => {
            println!("File created on {}", file_path);
        }
        Err(e) => {
            println!("Error to create file: {}", e);
        }
    }
}
