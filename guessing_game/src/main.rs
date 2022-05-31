use std::io::stdin;
use rand::Rng;
use rand::thread_rng;
use std::cmp::Ordering;


fn main() {

    let start_range = 0;
    let end_range = 5;
    
    let secret_number:u32 = thread_rng().gen_range(start_range..=end_range);

    println!("Adivinhe o número !");
    
    loop {

        let mut guess = String::new();

        println!("Entre com um número entre {} e {} para começar ) >> ", start_range, end_range);
        
        stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            
            Ordering::Less => println!("É um número maior"),
            Ordering::Greater => println!("É um número menor"),
            Ordering::Equal => {
                println!("You win!"); break;
            }
        }

    }
       
}
