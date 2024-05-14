mod file;

use file::{create, get_user_folder, read, read_dir};

fn main() {
    let path = get_user_folder().unwrap();

    let file_path = create(&path, &"te2st.txt").unwrap();

    read(&file_path);

    match read_dir(&path) {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Err"),
    }
}
