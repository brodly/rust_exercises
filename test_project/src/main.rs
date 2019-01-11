mod test_fn;

fn main() {
    let mut v = vec![1, 2, 3];

    println!("{:?}", v);

    for i in &mut v {
        println!("{}", i);
        *i += 50;
        println!("{}", i);
    }

    println!("{:?}", v);
    enum_test();
}

fn enum_test() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    let k = &row[1];

    println!("{:?}", k);

    test_fn::test();
}