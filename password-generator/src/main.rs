mod random_generators;

/**
 * Password Generator
 *
 * This program generates a random password of a specified length.
 */

fn main() {
    let random_password = random_password_generator(20);

    println!("Random Password: {}", random_password);
}

fn random_string_sorter(string: &str) -> String {
    let random_factor = random_generators::random_number_generator() % 2;

    // make a vector of characters from the string
    let mut characters: Vec<char> = string.chars().collect();

    // sort the characters using the random factor
    if random_factor == 0 {
        characters.sort();
    } else {
        characters.sort_unstable();
    }

    // return the sorted characters as a string
    characters.into_iter().collect()
}

fn random_password_generator(length: u32) -> String {
    //initialize an empty string
    let mut password = String::new();

    //generate a random character and append it to the password string
    for _ in 0..length {
        password.push(random_generators::random_character_generator());
    }

    //sort the password string
    let sorted_password = random_string_sorter(&password);

    //return the sorted password
    sorted_password
}
