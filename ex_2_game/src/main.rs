extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Adivinhe o número!");

    loop {
        println!("Digite seu palpite.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Falha ao ler entrada");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você disse: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("O número é maior."),
            Ordering::Greater => println!("O número é menor."),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
