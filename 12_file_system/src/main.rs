mod file;

use file::{create, get_user_folder, read};

fn main() {
    let path = get_user_folder().unwrap();

    let file_path = create(&path, &"te2st.txt").unwrap();

    read(&file_path);
}
