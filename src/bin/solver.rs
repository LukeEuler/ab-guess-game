use std::io;

use ab_guess_game::number::Number;

fn main() {
    let mut record_list = Vec::<Record>::new();
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let list: Vec<_> = guess.trim().split(' ').collect();
        if list.len() == 1 {
            if guess.trim().to_lowercase().eq("show") {
                show(&record_list);
                continue;
            }
            if guess.trim().to_lowercase().eq("shows") {
                shows(&record_list);
                continue;
            }
            println!("invalid command!");
            continue;
        }
        if list.len() != 2 {
            println!("invalid: {}", list[0]);
            continue;
        }

        let num = match Number::new(list[0]) {
            Ok(num) => num,
            Err(be) => {
                println!("{}", be);
                continue;
            }
        };

        if list[1].len() != 4 {
            println!("invalid: {}", list[1]);
            continue;
        }

        let aa = list[1].to_lowercase().chars().nth(1).unwrap();
        let bb = list[1].to_lowercase().chars().nth(3).unwrap();
        if aa != 'a' || bb != 'b' {
            println!("invalid: {}", list[1]);
            continue;
        }

        let mut a = list[1].chars().nth(0).unwrap().to_ascii_uppercase() as u8;
        if a < 48 || a > 52 {
            println!("invalid: {}", list[1]);
            continue;
        }
        let mut b = list[1].chars().nth(2).unwrap().to_ascii_uppercase() as u8;
        if b < 48 || b > 52 {
            println!("invalid: {}", list[1]);
            continue;
        }
        a -= 48;
        b -= 48;
        if a + b > 4 {
            println!("invalid: {}", list[1]);
            continue;
        }

        let item = Record { num, a, b };

        let mut valid = true;
        for temp in record_list.iter() {
            if temp.num.value != item.num.value {
                continue;
            }
            valid = false;
            println!("aleray exist: {} {}A{}B", temp.num.value, temp.a, temp.b);
            break;
        }
        if !valid {
            continue;
        }

        record_list.push(item);

        println!("now {}", record_list.len());
    }
}

fn show(record_list: &Vec<Record>) {
    println!("------- all records -------");
    for item in record_list.iter() {
        println!("{} {}A{}B", item.num.value, item.a, item.b);
    }
    println!("------- done -------");
    let mut list = vec![];
    for x in 1023..=9876 {
        match Number::new_with_number(x as u32) {
            Ok(item) => {
                list.push(item);
            }
            Err(_) => {}
        };
    }
    let mut new_list = vec![];
    for item in list {
        let mut ookk = true;
        for r in record_list {
            let (a, b) = r.num.ab_check(item.get_guess_number());
            if r.a != a as u8 || r.b != b as u8 {
                ookk = false;
                break;
            }
        }
        if ookk {
            new_list.push(item.clone());
        }
    }
    println!("{}", new_list.len());
}

fn shows(record_list: &Vec<Record>) {
    println!("------- all records -------");
    for item in record_list.iter() {
        println!("{} {}A{}B", item.num.value, item.a, item.b);
    }
    println!("------- done -------");
    let mut list = vec![];
    for x in 1023..=9876 {
        match Number::new_with_number(x as u32) {
            Ok(item) => {
                list.push(item);
            }
            Err(_) => {}
        };
    }
    let mut new_list = vec![];
    for item in list {
        let mut ookk = true;
        for r in record_list {
            let (a, b) = r.num.ab_check(item.get_guess_number());
            if r.a != a as u8 || r.b != b as u8 {
                ookk = false;
                break;
            }
        }
        if ookk {
            new_list.push(item.clone());
        }
    }
    println!("{}", new_list.len());
    for item in new_list {
        println!("{}", item.value);
    }
}

struct Record {
    pub num: Number,
    pub a: u8,
    pub b: u8,
}
