use rand::seq::SliceRandom;
use std::io;
use std::process;

fn random_color_array<'a>(base_colors: &'a [&'a str; 6], code_length: i32) -> [&'a str; 4] {
    let mut colors_to_match: [&str; 4] = Default::default();
    let mut rng = rand::thread_rng();

    for index in 0..code_length as usize {
        let random_element = base_colors.choose(&mut rng);

        match random_element {
            Some(element) => colors_to_match[index] = element,
            None => println!("Array is empty"),
        }
    }
    colors_to_match
}

fn validate_user_guess(colors_to_match: [&str; 4], user_guess: Vec<&str>) -> (i32, i32) {
    let zipped: Vec<_> = colors_to_match
        .iter()
        .zip(user_guess.iter())
        .collect();

    let correct_position: i32 = zipped
        .iter()
        .filter(|&(a, b)| a == b)
        .count() as i32;

    let mut correct_color: i32 = colors_to_match
        .iter()
        .filter(|x| user_guess.contains(x))
        .count() as i32;

    correct_color -= correct_position;
    (correct_position, correct_color)
}

fn main() {
    let base_colors: [&str; 6] = ["Red", "Green", "Yellow", "Blue", "Purple", "Orange"];
    let code_length: i32 = 4;
    let max_attemps: i32 = 10;
    let mut attempts: i32 = 0;

    let colors_to_match = random_color_array(&base_colors, code_length);

    println!("\nWelcome to the Mastermind Game!!\n\nAvailable colors: {}\nCode length: {}, Max attemps: {}", base_colors.join(", "), code_length, max_attemps);

    while attempts < max_attemps {
        println!(
            "Attempt: {}/{}. Enter your guess.",
            (attempts + 1),
            max_attemps
        );

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let user_guess: Vec<&str> = guess.trim().split(" ").collect();

        // Validating length of the user guess and contents
        if !(user_guess.iter().all(|&item| base_colors.contains(&item))
            && user_guess.len() == colors_to_match.len())
        {
            println!("Invalid Guess, make sure you have exactly four colors!");
            continue;
        }

        let (correct_position, correct_color): (i32, i32) =
            validate_user_guess(colors_to_match, user_guess);

        if correct_position == colors_to_match.len() as i32 {
            println!(
                "\n\nCongratulations!! You win!!\n\nCode: {}",
                colors_to_match.join(", ")
            );
            process::exit(0)
        }

        println!(
            "\n{} colors placed correctly!\n{} correct colors placed in the wrong position!\n\n",
            correct_position, correct_color
        );

        attempts += 1;
    }
    println!("\n\nNo more attempts, You lose!");
    println!("Code: {}", colors_to_match.join(", "));
}
