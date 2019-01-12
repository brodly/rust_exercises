use std::io;
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
    input.pop();

    let first_character = &input[0..1];

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
}
