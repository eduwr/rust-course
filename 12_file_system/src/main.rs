mod file;

use file::{create, get_user_folder};

fn main() {
    let path = get_user_folder().unwrap();
 
    create(&path, &"test.txt");
         
    
}
