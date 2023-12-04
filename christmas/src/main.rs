


fn main() {
    let mut count = 1;
    while count <= 12 {
        println!("On the {count} day of christmas my true love gave to me...");
        looper(count); 
        println!("");
        count += 1;
    }
}

fn looper(number: i32) {
    let mut i = number;
    while i > 0 {
        lyrics(i);
        i -= 1;
    }
}

fn lyrics(num: i32) {
    match num {
        1 => println!("A partridge in a pear tree"),
        2 => println!("Two turtle doves"),
        3 => println!("Three french hens"),
        4 => println!("Four calling birds"),
        5 => println!("Five golden rings"),
        6 => println!("Six geese a layin'"),
        7 => println!("Seven swans a swimmin'"),
        8 => println!("Eight maids a milkin'"),
        9 => println!("Nine ladies dancin'"),
        10 => println!("Ten lords a leapin'"),
        11 => println!("Eleven pipers pipin'"),
        12 => println!("Twelve drummers drummin'"),
        _ => println!("Something went wrong")
    }
}
