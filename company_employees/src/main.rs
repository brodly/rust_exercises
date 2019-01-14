use std::io;
use std::collections::HashMap;

fn main() {
    let mut company = HashMap::new();

    loop {
        let mut select = String::new();
        let mut name = String::new();
        let mut department = String::new();

        println!("Press 1 to input employees into department");
        println!("Press 2 to list employees from department");

        io::stdin().read_line(&mut select).unwrap();

        let select: u32 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please choose a number");
                continue;
            },
        };

        match select {
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

                io::stdin().read_line(&mut department).unwrap();
                department.pop();

                match company.get(&department) {
                    None => {
                        println!("No Department Found");
                        continue;
                    },
                    Some(employees) => {
                        println!("Employee List: {:?}", employees);
                        continue;
                    },
                }
            },
            _ => (),
        }
    }
}