
use std::io;
use rand::seq::SliceRandom;

fn main() {

    let base_colors: [&str; 6] = ["Red", "Green", "Yellow", "Blue", "Purple", "Orange"];
    let mut colors_to_match: [&str; 4] = Default::default();
    let mut user_guess = Vec::new();
    let code_length: i32 = 4;
    let max_attemps: i32 = 10;
    let mut attemps: i32 = 0;
    let mut guess = String::new();
    let mut correct_position: i32 = 0;
    let mut correct_color: i32 = 0;


    for index in 0..4{

        let mut rng = rand::thread_rng();

        let random_element = base_colors.choose(&mut rng);

        match random_element {
            Some(element) => colors_to_match[index] = element,
            None => println!("Array is empty"),
        }
    }

    println!("{:?}", colors_to_match);

    println!("\nWelcome to the Matermind Game!!\n\nAvailable colors: {}\nCode length: {}, Max attemps: {}", base_colors.join(", "), code_length, max_attemps);


    while attemps < max_attemps{
        let _ = &mut user_guess.clear();
        let _ = &mut guess.clear();
        correct_color = 0;
        correct_position = 0;


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        user_guess = guess.trim()
            .split(" ")
            .collect();

        if !(user_guess.iter().all(|&item| base_colors.contains(&item)) && user_guess.len() == colors_to_match.len()) {
            println!("Invalid Guess, make sure you have exactly four colors!");
            continue;
        }

        let zipped: Vec<_> = colors_to_match.iter()
            .zip(user_guess
            .iter())
            .collect();

        // Need improvement
        for i in zipped{
            if i.0 == i.1 {
                correct_position += 1;
            }
            // This does not work
            if colors_to_match.contains(i.1) {
                correct_color += 1;
            }
        }


        println!("{:?}, correct pos {}, correct color {}", user_guess, correct_position, correct_color);


        attemps += 1;
    }
}
