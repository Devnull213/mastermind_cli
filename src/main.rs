use rand::seq::SliceRandom;
use std::io;
use std::process;

fn main() {
    let base_colors: [&str; 6] = ["Red", "Green", "Yellow", "Blue", "Purple", "Orange"];
    let code_length: i32 = 4;
    let max_attemps: i32 = 10;
    let mut colors_to_match: [&str; 4] = Default::default();
    let mut attempts: i32 = 0;

    // Creating a random array witn colors
    for index in 0..code_length as usize {
        let mut rng = rand::thread_rng();
        let random_element = base_colors.choose(&mut rng);

        match random_element {
            Some(element) => colors_to_match[index] = element,
            None => println!("Array is empty"),
        }
    }

    println!("\nWelcome to the Matermind Game!!\n\nAvailable colors: {}\nCode length: {}, Max attemps: {}", base_colors.join(", "), code_length, max_attemps);

    while attempts < max_attemps {
        let mut guess = String::new();

        println!(
            "Attempt: {}/{}. Enter yrou guess.",
            (attempts + 1),
            max_attemps
        );

        // Processing user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let user_guess: Vec<_> = guess.trim().split(" ").collect();

        // Validating length of the user guess and contents
        if !(user_guess.iter().all(|&item| base_colors.contains(&item))
            && user_guess.len() == colors_to_match.len())
        {
            println!("Invalid Guess, make sure you have exactly four colors!");
            continue;
        }

        let zipped: Vec<_> = colors_to_match.iter().zip(user_guess.iter()).collect();

        let correct_position: i32 = zipped.iter().filter(|&(a, b)| a == b).count() as i32;

        let mut correct_color: i32 = colors_to_match
            .iter()
            .filter(|x| user_guess.contains(x))
            .count() as i32;

        correct_color -= correct_position;

        if correct_position == colors_to_match.len() as i32 {
            println!("\n\nCongratulations!! You win!\nCode was: {}", colors_to_match.join(", "));
            process::exit(0)
        }

        println!(
            "\n{} colors placed correctly!\n{} correct colors placed in the wrong position!\n\n",
            correct_position, correct_color
        );

        attempts += 1;
    }
    println!("\n\nNo more attempts, You lose!");
    println!("{}", colors_to_match.join(", "));
}
