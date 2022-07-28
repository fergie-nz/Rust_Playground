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
            },
            3 => {
                println!("you chose {christmas}");
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
