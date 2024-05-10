fn main() {
    ify();
    if_returning(true);
    if_returning(false);
    inity_loop();
    while_cond();
    for_iterator();
    match_cond("summer");
    match_cond("winter");
    match_cond("spring");
    match_cond("fall");
    match_cond("idk");
}

fn ify() {
    let numero = 99;

    if numero > 0 {
        println!("The number {} is positive", numero)
    } else if numero == 0 {
        println!("The number is zero")
    } else {
        println!("The number {} is negative", numero)
    }
}

fn if_returning(condition: bool) {
    let resultado = if condition {
        "The condition is true"
    } else {
        "condition is false"
    };

    println!("{}", resultado)
}

fn inity_loop() {
    let mut counter = 0;

    loop {
        println!("Count loop: {}", counter);
        if counter == 5 {
            break;
        }
        counter += 1;
    }
}

fn while_cond() {
    let mut counter = 0;

    while counter <= 5 {
        println!("Counter while: {}", counter);
        counter += 1;
    }
}

fn for_iterator() {
    for n in 0..4 {
        println!("the number is {}", n)
    }
    for n in 0..=4 {
        println!("the number is {}", n)
    }
}

fn match_cond(curr_season: &str) {
    match curr_season {
        "spring" => {
            println!("It's spring!");
        }
        "summer" => {
            println!("It's summer!");
        }
        "fall" => {
            println!("It's fall!");
        }
        "winter" => {
            println!("It's winter!");
        }
        _ => {
            println!("Invalid season!");
        }
    }
}
