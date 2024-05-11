fn main() {
    let nome = String::from("Hcode");

    let empresa = &nome;

    println!("Hello, {}!", nome);
    println!("Hello, {}!", empresa);
}
