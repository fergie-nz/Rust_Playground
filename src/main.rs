use std::io;

fn main() {
    println!("Welcome to function fun. Where we can demonstrate some function activity in Rust");

    loop {

        let temperature:String = "F_to_C";
        let fibonacci:String = "nth Fibonacci number";
        let christmas:String = "lyrics";

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
            }
            2 => {
                println!("you chose {temperature}");
            }
            3 => {
                println!("you chose {temperature}");
            }
            4 => {
                println!("you chose {temperature}");
            }
        }
        

    }
}
