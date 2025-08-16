use std::cmp::Ordering;
use std::io;

use rand::Rng;
fn main() {
    println!("Загадайте число");

     match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!(r#"Пожалуйста загадайте свое число"#);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Ошибка чтения строки");
    
    println!("Вы загадали: {}", guess);
}