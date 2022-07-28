use std::io;

fn main() {
    println!("Welcome to function fun. Where we can demonstrate some function activity in Rust");

    loop {

        let temperature:&str = "F_to_C";
        let fibonacci:&str = "nth Fibonacci number";
        let christmas:&str = "lyrics";

        println!("Choose your function:\n1 = {temperature}\n2 = nth {fibonacci}\n3 = {christmas}\n4 = quit");
        
        let mut selection = String::new();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");
        
        let selection: u32 = match selection
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You must enter a number");
                    continue;
                }
            };
        
        match selection {
            1 => {
                println!("you chose {temperature}");
                temp();
            },
            2 => {
                println!("you chose {fibonacci}");
                fib();
            },
            3 => {
                println!("you chose {christmas}");
                lyrics()
            },
            4 => {
                break;
            },
            _ => {continue}
        }
    }
}

fn temp() {
    println!("Enter the C value you want to convert");

    let mut tcelsius = String::new();

    io::stdin()
        .read_line(&mut tcelsius)
        .expect("Failed to read line");
    
    let tcelsius: i64 = match tcelsius
    .trim()
    .parse() {
        Ok(num) => num,
        Err(_) => {
            println!("You must enter a number");
            return;
        }
    };

    let tfarenheight:i64 = (tcelsius*9/5)+32;

    println!("{tcelsius}C converted to farenheit is {tfarenheight}F");
}

fn fib() {
    println!("Enter the n, and this will calculate the nth fibonacci number");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    
    let number: i64 = match number
    .trim()
    .parse() {
        Ok(num) => num,
        Err(_) => {
            println!("You must enter a number");
            return;
        }
    };

    let fibonacci = fibnum(number);
    println!("the {number}th fibonacci number is {fibonacci}");
    // println!("{fibonacci}")
}

fn fibnum(num:i64) -> i64 {
    if num < 1 {
        return 0;
    } else if num == 1 {
        return 1;
    } else {
        return {
            fibnum(num-1) + fibnum(num-2)
        };
    }   
}

fn lyrics() {
    for number in 1..12 {
        let mut suffix = String::new()
        if number = 1 {
            suffix = "st"
        } else if number = 2 {
            suffix = "nd"
        } else if number = 3 {
            suffix = "rd"
        } else {
            suffix = "th"
        }
        println!("On the {number}{suffix} day of Christmas, my true love sent to me");
        println!("a partridge and a pear tree")
    }
    println!("LIFTOFF!!!");
}