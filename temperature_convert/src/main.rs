use std::io;

fn main() {
    loop {
        println!("Press 1 for C to F");
        println!("Press 2 for F to C");

        let mut convert_to = String::new();

        io::stdin().read_line(&mut convert_to)
            .expect("Failed to read line");

        let convert_to: u32 = match convert_to.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{:?} is not a number", convert_to.trim());
                continue;
            },
        };

        match convert_to {
            1 => println!("Please enter temperature (in C) to convert to F"),
            2 => println!("Please enter temperature (in F) to convert to C"),
            _ => {
                println!("'{}' is not a valid choice", convert_to);
                continue;
            },
        };

        let mut temp = String::new();

        io::stdin().read_line(&mut temp)
            .expect("Failed to read line");

        let temp: i32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{:?} is not a valid temperature", temp.trim());
                continue;
            },
        };

        check_and_convert(convert_to, &temp);
    }
}

fn check_and_convert(convert_to: u32, temp: &i32) {
    match convert_to {
        1 => {
            let new_temp = ((9 / 5) * temp) + 32;
            println!("{}C is {}F", temp, new_temp);
        },
        2 => {
            let new_temp = ((temp - 32) * 5) / 9;
            println!("{}F is {}C", temp, new_temp);
        },
        _ => (),
    }
}