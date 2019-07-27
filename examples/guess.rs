extern crate ab_guess_game;

use std::io;

use ab_guess_game::number::{Number, Numbers};

fn main() {
    Numbers::new();
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let list: Vec<&str> = guess.trim().split_whitespace().collect();
        if list.len() != 2 {
            println!("can not check: {}", guess);
            continue;
        }

        let guess_number: Number = match Number::new(list[0]) {
            Ok(n) => n,
            Err(be) => {
                println!("{}", be);
                continue;
            }
        };

        println!("{:?}", guess_number.value);
        println!("{:?}", list);
    }
}
