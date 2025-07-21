use rand::Rng;

pub fn guess_number(io: &mut std::io::Stdin) {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Guess the number!");
    println!("Input your guess: ");

    let mut guess = String::new();
    
    io.read_line(&mut guess)
        .expect("Failed to read line");

    if secret_number == guess.trim().parse::<i32>().expect("Please type a number!") {
        println!("You win!");   
    } else {
        println!("You lose!");
    }

    println!("You Guessed: {guess}");
    println!("The secret number is: {secret_number}");
}