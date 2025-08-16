use std::io;
fn main() {
    println!("Загадайте число");

    println!(r#"Пожалуйста загадайте свое число"#);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Ошибка чтения строки");
    
    println!("Вы загадали: {}", guess);
}