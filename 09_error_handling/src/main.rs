use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    // func_with_panic(0);

    println!("===========// Panic //============");
    let result = std::panic::catch_unwind(|| {
        let a = func_with_panic(0);

        Ok::<i32, &str>(a)
    });

    match result {
        Ok(val) => {
            println!("you got it {}", val.unwrap());
        }
        Err(_) => {
            println!("you miss!");
        }
    }
    println!("===========// Result //============");
    // let res = read_file("/home/eduardo/eduardo/rust-course/09_error_handling/src/main.rs");
    let res = read_file("/home/edue/09_error_handling/src/main.rs");

    match res {
        Ok(content) => {
            println!("Content: {}", content);
        }
        Err(_) => {
            println!("Errou");
        }
    }

    println!("===========// Option //============");
    let division_result = divide(1 as f64, 2 as f64);

    match division_result {
        Some(r) => {
            println!("Result is {}", r);
        }
        None => {
            println!("not possible to divide by zero")
        }
    }
}

fn func_with_panic(val: i32) -> i32 {
    if val == 0 {
        panic!("Oh my god!")
    }

    val
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;

    let mut content = String::new();

    let _ = file.read_to_string(&mut content);

    Ok(content)
}

fn divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividend / divisor)
    }
}
