use std::io;

fn main() {
    
    loop{
        println!("Please select one of the following convertions: \n 1) from Celsius to Fahrenheit \n 2) from Fahrenheit to Celsius\n" );
        let mut choice = String :: new();

        io::stdin().read_line(&mut choice).expect("failed to read line!");

        let choice : u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number.");
                continue;
            }
        };

        loop{
            println!("Please enter temperature");
            let mut temperature = String :: new();
    
            io::stdin().read_line(&mut temperature).expect("failed to read line!");
    
            let temperature : f32 = match temperature.trim().parse(){
                Ok(num) => num,
                Err(_) => {
                    println!("Please input a number.");
                    continue;
                }
            };

            match choice {
                    1 => println!("{} degrees Celsius are {} degrees Fahrenheit", temperature,  celsius_to_fahrenheit(temperature)),
                    2 => println!("{} degrees Fahrenheit are {} degrees Celsius", temperature,  fahrenheit_to_celsius(temperature)),
                    _ => {println!("invalid choice");
                             continue;
                    }            
            };
            return;
        }
    }
}

fn celsius_to_fahrenheit(celsius : f32) -> f32{
    return (celsius * 1.8) + 32.0;
} 

fn fahrenheit_to_celsius(fahrenheit : f32) -> f32 {
    return (fahrenheit - 32.0) * 0.5556;
} 
