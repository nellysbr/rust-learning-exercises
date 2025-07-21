mod modules {
    pub mod guess_number;
    pub mod arrays_module;
    pub mod tuples_module;
}

use modules::arrays_module::arrays_m;
use modules::guess_number::guess_number;
use modules::tuples_module::tuples_m;
use std::io;

fn main() {
    loop {
        println!("\nChoose the exercise: ");
        println!("1. Guess the number");
        println!("2. Arrays");
        println!("3. Tuples");
        println!("4. Exit");

        let mut exercise = String::new();

        io::stdin()
            .read_line(&mut exercise)
            .expect("Failed to read line");

        let exercise = exercise.trim().parse::<i32>().expect("Please type a number!");

        match exercise {
            1 => guess_number(&mut io::stdin()),
            2 => arrays_m(&mut io::stdin()),
            3 => tuples_m(),
            4 => break,
            _ => println!("Invalid exercise!"),
        }
    }
}
