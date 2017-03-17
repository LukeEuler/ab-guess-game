use std::io;

use ab_guess_game::number::{create_secret_number, Number};

fn main() {
    let rules = include_str!("../assets/rules.txt");
    println!("{}", rules);
    let secret_number = create_secret_number();
    let mut step = 0;

    loop {
        println!("Guess the number!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // see the answer and end the game
        if guess.trim() == "answer" {
            let mut answer = 0;
            for i in secret_number.iter() {
                answer = answer * 10 + i;
            }
            println!("{}", answer);
            break;
        }

        let guess_number = match Number::new(&guess) {
            Ok(num) => num,
            Err(be) => {
                println!("{}", be);
                continue;
            }
        };

        step += 1;
        let (a, b) = guess_number.ab_check(secret_number);
        if a == 4 {
            println!("Step: {step}. You win!");
            break;
        }
        println!(
            "Step: {step}. You guessed: {}. {a}A{b}B",
            guess_number.value
        );
    }
}
