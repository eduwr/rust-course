fn main() {
    nome_da_funcao();

    let a = com_return();

    println!("O valor retornado é : {}", a);

    let b = sem_return();

    println!("O valor retornado é : {}", b);

    let maior = maior_valor(a.into(), i32::from(b));

    println!("O maior valor entre {} e {} é {}", a, b, maior);


    let d = 6;

    let resultado_increment = increment(d);

    println!("Número original é {}, o valor incrementado é {}", d, resultado_increment);

}

fn nome_da_funcao() {
    println!("Hello, world!");
}

fn com_return() -> i8 {
    return 3;
}

fn sem_return() -> i8 {
    5
}

fn maior_valor(a: i32, b: i32) -> i32 {
    {
        if a > b {
            a
        } else {
            b
        }
    }
}


fn increment(mut a: i32) -> i32 {
    a += 1;
    a
}
