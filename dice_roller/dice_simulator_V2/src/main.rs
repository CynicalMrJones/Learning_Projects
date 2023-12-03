
use rand::Rng;
use std::env;

fn main() {

    let args: Vec<_> = env::args().collect();
    let number = args[1].parse::<i32>().expect("Error: Unable to parse first argument");
    let dice = args[2].parse::<String>().expect("Error Unable to parse second argument");
    let answer = matcher(&dice);

    let array = roller(number, answer);
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;
        while j < number {
            sum = array[j as usize] + sum;
                j = j + 1;
        }

        while i < number {
            println!("{}", array[i as usize]);
            i += 1
        };
        println!("The sum of all number is equal to: {}", sum)
}

fn matcher(dice: &String) -> i32{
    match dice.as_str() {
        "d2" => 2,
        "d4" => 4,
        "d6" => 6,
        "d8" => 8,
        "d10" => 10,
        "d20" => 20,
        _ => word_split(dice)
    }
}

fn word_split(s: &String) -> i32{
   let bytes = s.as_bytes();

   for(_i, &items) in bytes.iter().enumerate(){
       if items == b'd'{
           return s[1..].parse::<i32>().expect("Error: Unable to parse")
       }
   }
   s[..].parse::<i32>().expect("Error: Unable to parse")
}

fn roller(num: i32, dice_num: i32) -> [i32; 100] {
    let mut i = 0;
    let mut answer: [i32; 100] = [0; 100];
    loop {
        let non_normal_dice = rand::thread_rng().gen_range(1..=dice_num);
        answer[i as usize] = non_normal_dice;
        i += 1;
        if i > num {
            return answer;
        }
    }
}

