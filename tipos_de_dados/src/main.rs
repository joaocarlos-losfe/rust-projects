fn main() {

    let guess: u32 = "42".parse().expect("Not a number!"); //covertendo string númerica em numero;
    println!("{}", guess);

    let x:u64 = 100_000_150;
    eprintln!("x: {}", x);

}
