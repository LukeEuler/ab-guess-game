extern crate rand;

use std::io;
use rand::Rng;

fn creat_secreat_number() -> [u32; 4] {
    let mut secret_number: [u32; 4] = [0, 0, 0, 0];
    secret_number[0] = rand::thread_rng().gen_range(1, 10);
    loop {
        secret_number[1] = rand::thread_rng().gen_range(0, 10);
        if secret_number[1] != secret_number[0] {
            break;
        }
    }
    loop {
        secret_number[2] = rand::thread_rng().gen_range(0, 10);
        if secret_number[2] == secret_number[0] {
            continue;
        } else if secret_number[2] == secret_number[1] {
            continue;
        } else {
            break;
        }
    }
    loop {
        secret_number[3] = rand::thread_rng().gen_range(0, 10);
        if secret_number[3] == secret_number[0] {
            continue;
        } else if secret_number[3] == secret_number[1] {
            continue;
        } else if secret_number[3] == secret_number[2] {
            continue;
        } else {
            break;
        }
    }
    secret_number
}

fn vaild_check(guess: u32) -> bool {
    let d = guess % 10;
    let c = (guess / 10) % 10;
    let b = (guess / 100) % 10;
    let a = guess / 1000;
    if a >= 10 || a == 0 {
        return false;
    }
    if a == b || a == c || a == d || b == c || b == d || c == d {
        return false;
    }
    true
}

fn ab_check(guess: u32, secret_number: [u32; 4]) -> [u32; 2] {
    let guess_number: [u32; 4] = [guess / 1000, (guess / 100) % 10, (guess / 10) % 10, guess % 10];
    let (mut a, mut b) = (0, 0);
    for i in 0..4 {
        if secret_number[i] == guess_number[i] {
            a += 1;
        }

        for j in 0..4 {
            if secret_number[i] == guess_number[j] {

                b += 1;
            }
        }
    }
    b -= a;
    [a, b]
}

fn main() {
    println!("AB猜数游戏");
    println!("规则：猜一个四位数，各位数值不等");
    println!("提示：数值且位置正确记作A，数值正确位置不对记作B");
    println!("请用最少的步数猜对");
    println!("");
    let secret_number = creat_secreat_number();
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

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if !vaild_check(guess) {
            continue;
        }

        step += 1;
        let ab = ab_check(guess, secret_number);
        println!("Step: {}. You guessed: {}. {}A{}B",
                 step,
                 guess,
                 ab[0],
                 ab[1]);
        if ab[0] == 4 {
            break;
        }
    }
}
