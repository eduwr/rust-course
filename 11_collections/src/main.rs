use std::collections::HashMap;

fn vector() {
    let list: [u8; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", list);

    let mut numbers: Vec<u8> = Vec::new();

    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);

    println!("{:?}", numbers);

    for n in numbers {
        println!("{}", n);
    }
}

fn string() {
    let mut text = String::new();

    println!("{}", text);
    text = String::from("Eduardo");
    text.push_str(" Wronscki");
    println!("{}", text);
}

fn hashmap() {
    let mut map = HashMap::new();

    map.insert("nome".to_string(), "Eduardo");
    map.insert("url".to_string(), "www.google.com");
    map.insert("url".to_string(), "www.google.com.br");

    println!("{:?}", map);

    match map.get(&"url".to_string()) {
        Some(val) => {
            println!("{}", val);
        },
        None => {
            println!("Not found");
        }
    }
}

fn main() {
    vector();

    println!("========================");

    string();

    println!("========================");

    hashmap();
}
