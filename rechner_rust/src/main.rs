use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    let num = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Nope");


        let input: u32 = input.trim().parse().expect("Please type a number!");

        match input.cmp(&num){
            Ordering::Less => println!("Zu klein"),
            Ordering::Greater => println!("Zu groß"),
            Ordering::Equal => {println!("Genau richtig"); break;}
        }
    }

}
