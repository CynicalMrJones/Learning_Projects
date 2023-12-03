
use rand::Rng;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let number = args[1].parse::<i32>().expect("Please enter a valid number");
    let dice = args[2].parse::<String>().expect("Please enter a valid dice");
    looper(number, &dice);
    }

fn looper(number: i32, dice: &String) {
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;
    if dice.to_string() == "d4" {
        let result = d4_roller(number);
        while j < number {
            sum = result[j as usize] + sum;
                j = j + 1;
        }
        while i < number {
            println!("{}", result[i as usize]);
            i = i +1 ;
        };
        println!("The sum of all numbers is equal too: {}", sum)
    }
    else if dice.to_string() == "d2" {
        let result = d2_roller(number);
        while j < number {
            sum = result[j as usize] + sum;
                j = j + 1;
        }
        while i < number {
            println!("{}", result[i as usize]);
            i = i +1 ;
        };
        println!("The sum of all numbers is equal to: {}", sum);
        while i < number {
            println!("{}", result[i as usize]);
            i = i +1 ;
        }
    }
    else if dice.to_string() == "d6"{
        let result = d6_roller(number);
        while j < number {
            sum = result[j as usize] + sum;
                j = j + 1;
        }
        while i < number {
            println!("{}", result[i as usize]);
            i = i +1 ;
        };
        println!("The sum of all numbers is equal to: {}", sum);
        while i < number {
            println!("{}", result[i as usize]);
            i = i +1 ;
        }
    }
    else if dice.to_string() == "d8" {
        let result = d8_roller(number);
        while j < number {
            sum = result[j as usize] + sum;
                j = j + 1;
        }
        while i < number {
            println!("{}", result[i as usize]);
            i = i +1 ;
        };
        println!("The sum of all numbers is equal to: {}", sum);
        while i < number {
            println!("{}", result[i as usize]);
            i = i +1 ;
        }
    }
    else if dice.to_string() == "d10" {
        let result = d10_roller(number);
        while j < number {
            sum = result[j as usize] + sum;
                j = j + 1;
        }
        while i < number {
            println!("{}", result[i as usize]);
            i = i +1 ;
        };
        println!("The sum of all numbers is equal to: {}", sum);
        while i < number {
            println!("{}", result[i as usize]);
            i = i +1 ;
        }
    }
    else if dice.to_string() == "d20" {
        let result = d20_roller(number);
        while j < number {
            sum = result[j as usize] + sum;
                j = j + 1;
        }
        while i < number {
            println!("{}", result[i as usize]);
            i = i +1 ;
        };
        println!("The sum of all numbers is equal to: {}", sum);
        while i < number {
            println!("{}", result[i as usize]);
            i = i +1 ;
        }
    }
    else {
        println!("Please enter a valid die");
        println!("Valid dice include...");
        println!("d2, d4, d6, d8, d10, d20")
    }

}

fn d4_roller(num: i32) -> [i32; 100] {
    let mut i = 0;
    let mut answer: [i32; 100] = [0; 100];
    while i < num {
        let d4 = rand::thread_rng().gen_range(1..=4);
        answer[i as usize] = d4;
        i = i+1;
    }
    return answer;
}

fn d2_roller(num: i32) -> [i32; 100] {
    let mut i = 0;
    let mut answer: [i32; 100] = [0; 100];
    while i < num {
        let d4 = rand::thread_rng().gen_range(1..=2);
        answer[i as usize] = d4;
        i = i+1;
    }
    return answer;
}

fn d6_roller(num: i32) -> [i32; 100] {
    let mut i = 0;
    let mut answer: [i32; 100] = [0; 100];
    while i < num {
        let d6 = rand::thread_rng().gen_range(1..=6);
        answer[i as usize] = d6;
        i = i+1;
    }
    return answer;
}

fn d8_roller(num: i32) -> [i32; 100] {
    let mut i = 0;
    let mut answer: [i32; 100] = [0; 100];
    while i < num {
        let d8 = rand::thread_rng().gen_range(1..=8);
        answer[i as usize] = d8;
        i = i+1;
    }
    return answer;
}
fn d10_roller(num: i32) -> [i32; 100] {
    let mut i = 0;
    let mut answer: [i32; 100] = [0; 100];
    while i < num {
        let d6 = rand::thread_rng().gen_range(1..=10);
        answer[i as usize] = d6;
        i = i+1;
    }
    return answer;
}
fn d20_roller(num: i32) -> [i32; 100] {
    let mut i = 0;
    let mut answer: [i32; 100] = [0; 100];
    while i < num {
        let d20 = rand::thread_rng().gen_range(1..=20);
        answer[i as usize] = d20;
        i = i+1;
    }
    return answer;
}
