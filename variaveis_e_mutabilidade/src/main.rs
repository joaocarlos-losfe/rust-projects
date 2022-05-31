
fn main() {
    
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);


    let k = 5;

    let k = k + 1;

    {
        let k = k * 2;
        println!("\nThe value of x in the inner scope is: {}", k);
    }
    println!("The value of x is: {}", k);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);



}
