use std::io;

fn main() {

    let mut n1 = 0;
    let mut n2 = 1;
    let mut next_term;

    loop{

        println!("Enter the number of iterations");
        let mut iterations = String :: new();

        io::stdin().read_line(&mut iterations).expect("failed to read line!");
    
        let iterations : u32 = match iterations.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("This is not a number. Try again.");
                continue; 
            }
        };
        
        println!("{}\n{} ", n1, n2);

        for _numbers in 2..iterations{
            next_term = n1 + n2;
            println!("{} ", next_term);  
            n1 = n2;
            n2 = next_term;         
        }

        return
    }
}
