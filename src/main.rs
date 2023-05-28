use std::env;

use rand::Rng;
// use clipboard::{ClipboardContext, ClipboardProvider};

/**
 * provide an array and get a random element from it
 * 
 * password_keys: &[String] = the provided input array
 * current_index: usize = for what index the random character is required
 * returns = a random element from the input array
 */
fn get_random_character(
    password_keys: &Vec<&str>,
    current_index: usize
) -> String {
    let mut rng = rand::thread_rng();

    // we can not have few characters at the first index
    let index = if current_index == 0 && password_keys.len() > 62 {
        rng.gen_range(0..62) // 26 + 26 (alphas) + 10 numbers...
    } else {
        rng.gen_range(0..password_keys.len())
    };

    // match and return a random character each time...
    match password_keys.get(index) {
        Some(num) => num.to_string(),
        None => "0".to_string()
    }
}

/**
 * password generator using a provided list of characters and a limited password length
 * this function uses a random character generator to generate random characters and then concat them
 * that's all
 * 
 * password_keys: &[String] = the provided input array
 * password_length: usize = size of the password
 * return a random password each time...
*/
fn generate_password(
    password_keys: Vec<&str>,
    password_length: usize
) -> String {
    let mut password: String = String::from("");

    // itterate for a limited password length
    // and get a random character each time and concat that
    for index in 0..password_length {
        let ran_char: String = get_random_character(&password_keys, index);
        password = format!("{}{}", password, ran_char);
    }

    password
}

/**
 * copy any given string/password in this case to the system clipboard
 */
// fn copy_password_to_clipboard(password: String) {
//     // Create a new clipboard context
//     let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

//     // Attempt to copy the string to the clipboard
//     if let Err(err) = ctx.set_contents(password.to_owned()) {
//         eprintln!("Failed to set clipboard contents: {}", err);
//     } else {
//         println!("Password copied to clipboard!");
//     }
// }

fn main() -> Result<(), String> {
    // provided a string list to generate password from
    let password_keys = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "0",
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
        "@", "#", "$", "%", "^", "&", "*", "(", ")", "+", "=", "[", "]", ";", "-", ".", "?", ":"
    ];

    // get inputs from CLI
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // default password length will be 12
    let mut password_length: usize = 12;

    // if any password length is provided then use it...
    // but after validating it, if is a number or not...
    if args.len() > 1 {
        if args[1].clone().to_string().chars().all(char::is_numeric) {
            password_length = args[1].clone().to_string().trim().parse::<usize>().unwrap();
        } else {
            panic!("Please enter a valid length of password!");
        }
    }

    // generate -> print -> copy to clipboard
    let generated_password = generate_password(password_keys, password_length);
    println!("Generated Password: {}", generated_password);

    // copy_password_to_clipboard(generated_password);

    Ok(())
}