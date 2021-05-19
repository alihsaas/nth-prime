use std::io;

mod lib;

fn main() {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Invalid input");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let n = input.parse().expect("problem parsing usize");

        println!("{}", lib::get_nth_prime(n));
    }
}
