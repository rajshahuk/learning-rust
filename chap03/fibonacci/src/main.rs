use std::io;

fn main() {
    println!("Which position in the Fibonacci sequence would you like to generate up to?");
    let mut position = String::new();
    io::stdin().read_line(&mut position).expect("Need to enter something here!");
    let position :i64 = position.trim().parse().expect("It needs to be a number!");
    let mut previous :i64 = 0;
    let mut next :i64 = 1;
    for _ in 0..position {
        print!("{} ", previous);
        let sum = previous + next;
        previous = next;
        next = sum;
    }
    println!("\nFinished");
}
