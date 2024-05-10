enum Fruits {
    Apple,
    Banana,
    Strawberry,
    Acai,
}

enum Coordinates {
    TwoCoord(i32, i32),
    ThreeCoord(i32, i32, i32),
}

struct Person {
    name: String,
    age: i8,
    height: f32,
}

trait Polygon {
    fn area(&self) -> u32;

    fn new(height: u32, width: u32) -> Self;
}

struct Rectangle {
    height: u32,
    width: u32,
}

impl Polygon for Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn new(height: u32, width: u32) -> Self {
        Rectangle { width, height }
    }
}

fn main() {
    enumeration(Fruits::Acai);
    enumeration(Fruits::Apple);
    enumeration(Fruits::Banana);
    enumeration(Fruits::Strawberry);

    let coord2d = Coordinates::TwoCoord(5, 2);
    let coord3d = Coordinates::ThreeCoord(5, 2, 10);

    match_coord(coord2d);
    match_coord(coord3d);

    structure();

    rectangle_area(Rectangle::new(10, 20));
}

fn enumeration(fruit: Fruits) {
    match fruit {
        Fruits::Acai => println!("It's an acai!"),
        Fruits::Apple => println!("It's an Apple!"),
        Fruits::Banana => println!("It's a Banana!"),
        Fruits::Strawberry => println!("It's an Strawberry!"),
    }
}

fn match_coord(coord: Coordinates) {
    match coord {
        Coordinates::ThreeCoord(x, y, z) => println!("Coord 3d {}, {}, {}", x, y, z),
        Coordinates::TwoCoord(x, y) => println!("Coord 3d {}, {}", x, y),
    }
}

fn structure() {
    let eduardo = Person {
        age: 35,
        name: String::from("Eduardo"),
        height: 1.73,
    };

    println!("Name {}", eduardo.name);
    println!("Age {}", eduardo.age);
    println!("Height {}", eduardo.height);
}

fn rectangle_area(rect: Rectangle) {
    println!("Rectangle area is {}", rect.area());
}
