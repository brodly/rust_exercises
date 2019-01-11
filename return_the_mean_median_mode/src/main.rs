use std::collections::HashMap;

fn main() {
    let mut list = vec![1, 2, 7, 10, 3, 4, 6, 12, 15, 13, 14, 18, 1, 2, 2, 3];
    println!("The mean is {}", mean(&list));
    println!("The median is {}", median(&mut list));
    println!("The mode is {}", mode(&list));
}

fn mean (list: &Vec<u32>) -> f32 {
    let mut _average = 0;
    let l = list.len();

    for number in list {
        _average += number;
    }

    return _average as f32 / l as f32;
}

fn median (list: &mut Vec<u32>) -> u32 {
    list.sort();
    list[list.len() / 2]
}

fn mode (list: &Vec<u32>) -> u32 {
    let mut map = HashMap::new();

    for number in list {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut res_key: u32 = 0;
    let mut res_value: u32 = 0;

    for (key, value) in &map {
        if value > &res_value {
            res_value = *value;
            res_key = **key;
        }
    }

    res_key
}
