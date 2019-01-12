use std::io;
use std::collections::HashMap;

fn main() {
    // init company hashmap and input var
    let mut company: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut input = String::new();

    // Get input string and set it to input var
    io::stdin().read_line(&mut input).unwrap();

    // Remove newline character
    input.pop();

    // convert insert to str
    input = input.as_str();

    // check if department exists
    fn get_employees(department) -> Vec {
        match company.get(department) {
            // if no department exists
            None => {
                // create new vector with input as employee
                let input_vec = vec![input];

                // create department within company
                company.insert(department, input_vec);
            }

            // If list of employees exists
            Some(v) => {
                // push input var into vector
            }
        }
    }

    // insert


    let mut eng_vec = vec![];



    company.insert("engineering", eng_vec);

    println!("{:?}", company);
}

// {
//     engineering: ["Josh", "Jake"],
//     sales: ["Jorge", "Peter"],
// }