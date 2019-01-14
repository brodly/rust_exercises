use std::io;
use std::collections::HashMap;

fn main() {
    let mut company = HashMap::new();

    loop {
        let mut input = String::new();
        let mut name = String::new();
        let mut department = String::new();

        println!("Press 1 to list employees from department");
        println!("Press 2 to input employees into department");

        io::stdin().read_line(&mut input).unwrap();

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please choose a number");
                continue;
            },
        };

        match input {
            1 => {
                println!("Input Employee Name");

                io::stdin().read_line(&mut name).unwrap();
                name.pop();

                println!("Input Department");

                io::stdin().read_line(&mut department).unwrap();
                department.pop();

                match company.get_mut(&department) {
                    None => {
                        let mut new_employees = Vec::new();
                        new_employees.push(name);
                        company.insert(department, new_employees);
                        println!("{:?}", company);
                        continue;
                    },
                    Some(employees) => {
                        employees.push(name);
                        println!("{:?}", company);
                        continue;
                    }
                }
            },
            2 => {
                println!("Input Department Name");
            },
            _ => (),
        }
    }
}