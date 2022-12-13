use std::io;

fn main() {
    println!("Bem vindo a calculadora de equações do segundo grau");
    println!("Informe os valores da equação (ax²+bx+c=0)");

    let a = get_int32("a");
    let b = get_int32("b");
    let c = get_int32("c");

    let a_to_print = if a == 1 {
        "".to_string()
    } else {
        a.to_string()
    };
    let b_to_print = if b == 1 {
        "+".to_string()
    } else if b >= 0 {
        format!("+{}", b)
    } else {
        b.to_string()
    };
    let c_to_print = if c == 0 {
        "".to_string()
    } else if c > 0 {
        format!("+{}", c)
    } else {
        c.to_string()
    };

    println!("a equação a ser calculada é: {}x²{}x{}=0", a_to_print, b_to_print, c_to_print);

    let delta = (b * b) - (4 * a * c);
    println!("delta: {}", delta);

    let delta = (f32::sqrt((delta) as f32)) as i32;

    let x1 = ((b * -1) + delta) / (2 * a);
    let x2 = ((b * -1) - delta) / (2 * a);

    println!("x': {}", x1);
    println!("x\": {}", x2);
}

fn get_int32(value_name: &str) -> i32 {
    loop {
        let mut value = String::new();
        println!("Digite o valor de \"{}\": ", value_name);

        let error = format!("Falha ao ler o valor de \"{}\"", value_name);
        io::stdin().read_line(&mut value)
            .expect(&error);
        let value: i32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return value
    }
}
