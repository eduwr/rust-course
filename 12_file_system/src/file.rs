use std::{
    env,
    fs::File,
    io::{Read, Write},
};

pub fn get_user_folder() -> Option<String> {
    if let Some(home_path) = env::var_os("HOME") {
        Some(home_path.into_string().unwrap())
    } else {
        None
    }
}

pub fn create(path: &str, file_name: &str) -> Option<String> {
    println!("path: {}", path);
    println!("file_name: {}", file_name);

    let file_path = format!(r"{}/{}", path, file_name);
    println!("file_path: {}", file_path);

    match File::create(&file_path) {
        Ok(mut file) => {
            println!("File created on {}", file_path);

            let content = "Hello Rust";
            match file.write_all(content.as_bytes()) {
                Ok(_) => {
                    println!("Content added");
                }
                Err(_) => {
                    println!("Content not added")
                }
            }

            return Some(file_path);
        }
        Err(e) => {
            println!("Error to create file: {}", e);
            return None;
        }
    }
}

pub fn read(file_path: &str) {
    match File::open(file_path) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            println!("File opened: {}", content);
        }
        Err(e) => {
            println!("Could not read the file: {}", e);
        }
    }
}
