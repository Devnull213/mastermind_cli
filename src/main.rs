
use rand::seq::SliceRandom;

fn main() {

    let base_colors: [&str; 6] = ["Red", "Green", "Yellow", "Blue", "Purple", "Orange"];
    let mut colors_to_match: [&str; 4] = Default::default();
    let code_length: i32 = 4;
    let max_attemps: i32 = 10;


    for index in 0..4 {

        let mut rng = rand::thread_rng();

        let random_element = base_colors.choose(&mut rng);

        match random_element {
            Some(element) => colors_to_match[index] = element,
            None => println!("Array is empty"),
        }
    }

    println!("Welcome to the Matermind Game");
    println!("Available colors: {}", base_colors.join(", "));
    println!("Code length: {}, Max attemps: {}", code_length, max_attemps);

}
