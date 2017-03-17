use rand::Rng;

pub fn create_secret_number() -> [u32; 4] {
    let mut secret_number: [u32; 4] = [0, 0, 0, 0];
    secret_number[0] = rand::thread_rng().gen_range(1..10);
    for index in 1..4 {
        'outer: loop {
            secret_number[index] = rand::thread_rng().gen_range(0..10);
            for i in 0..index {
                if secret_number[i] == secret_number[index] {
                    continue 'outer;
                }
            }
            break;
        }
    }
    secret_number
}

#[derive(Copy, Clone)]
pub struct Number {
    pub value: u32,
}

impl Number {
    pub fn new(content: &str) -> Result<Number, String> {
        let num: u32 = match content.trim().parse() {
            Ok(num) => num,
            Err(_) => return Err(format!("can not parse {} as u32", content.trim())),
        };
        Number::new_with_number(num)
    }

    fn new_with_number(num: u32) -> Result<Number, String> {
        if !valid_check(num) {
            return Err(format!("invalid number {}", num));
        }
        Ok(Number { value: num })
    }

    fn get_guess_number(&self) -> [u32; 4] {
        let guess_number: [u32; 4] = [
            self.value / 1000,
            (self.value / 100) % 10,
            (self.value / 10) % 10,
            self.value % 10,
        ];
        guess_number
    }

    pub fn ab_check(&self, secret_number: [u32; 4]) -> (u32, u32) {
        let guess_number = self.get_guess_number();
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
        (a, b)
    }
}

fn valid_check(guess: u32) -> bool {
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

#[derive(Clone)]
pub struct Numbers {
    content: Vec<Number>,
}

#[allow(dead_code)]
impl Numbers {
    pub fn new() -> Numbers {
        let mut result: Numbers = Numbers {
            content: Vec::new(),
        };
        for value in 1023..9877 {
            match Number::new_with_number(value as u32) {
                Ok(n) => {
                    result.content.push(n);
                }
                Err(_) => {}
            };
        }
        // for value  in &result.content {
        //      println!("{}", value.value);
        // }
        // println!("{}", result.content.len());
        result
    }

    pub fn guess(&mut self, _num: Number, (_a, _b): (u32, u32)) {}
}
