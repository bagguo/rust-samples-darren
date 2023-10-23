use core::num;
use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    // print_test();
    // greet_world();
    // most_used();
    // guess_num();
    shadowing();
}

fn print_test() {
    println!("hi rust");

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number = 1); // 00001
                                          // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "fixed name");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region); // !: marco宏
    }
}

fn most_used() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        //声明一个vec集合
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug:{:?} -> {:?}", record, fields)
        }

        let name = fields[0];

        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}

fn guess_num() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new(); //:: 语法表明 new 是 String 类型的一个 关联函数（associated function）
                                       // let apples = 5; //默认不可变
                                       // let mut bananas = 5; //可变的

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess:u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    //shadowing can change types
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    //mut can change types
    let mut spaces = "   ";
     spaces = spaces.len();
    println!("{spaces}");
}