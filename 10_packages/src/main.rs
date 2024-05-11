use math::operations::subtract as fluffytract;
use math::operations::sum;
use rand::random;

mod math;

fn main() {
    let a = random::<i8>();
    let b = random::<i8>();

    println!("{} + {} = {}", a, b, sum(a as i32, b as i32));
    println!("{} - {} = {}", a, b, fluffytract(a as i32, b as i32));
}
