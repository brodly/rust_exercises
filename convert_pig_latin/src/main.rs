use std::io;
use std::collections::HashMap;
#[macro_use] extern crate maplit;

fn main() {
    let vowel_map = hashmap![
        "a" => 1,
        "e" => 1,
        "i" => 1,
        "o" => 1,
        "u" => 1,
    ];

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    // slice the first_character off the input
    let mut first_character = &input[0..1];

    match vowel_map.get(&first_character) {
        Some(_) => {
            input = input + "-hay";
            println!("{}", input);
        },
        None => {
            let pig_latin = first_character.to_owned() + "ay";
            let first = input[1..].to_owned();
            println!("{}", first + "-"+ &pig_latin);
        }
    }

    // pass reference of first_character to vowel_map and check if key exists in vowel_map
        // on ok
            // take overship of input and concat "-hay"
            // return input
        // on error
            // take owenrship of first_character and concat string "ay"
            // take overshiper of input and concat "-" concat first_character
            // return input
}
