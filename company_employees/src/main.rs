use std::io;
use std::collections::HashMap;

fn main() {
    let mut company: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut input = String::new();

    // loop {
        io::stdin().read_line(&mut input).unwrap();
        input.pop();

        let input_vec = vec![input.as_str()];

        company.insert("engineering", input_vec);
        let eng = company.get("engineering");
        let mut eng_vec = vec![];

        if let Some(v) = eng {
            eng_vec = v.to_vec();
        }

        company.insert("engineering", eng_vec);

        println!("{:?}", company);
    // }
}

// {
//     engineering: ["Josh", "Jake"],
//     sales: ["Jorge", "Peter"],
// }