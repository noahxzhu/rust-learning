fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v1 = vec![1, 2, 3];
    // println!("Hello, world!");

    // let mut v = Vec::new();
    // v.push(1);
    // v.push(5);
    // v.push(6);
    // v.push(7);

    // let v = vec![1, 2, 3, 4, 5];
    //
    // let third: &i32 = &v[2];
    // println!("The third element is {third}");
    //
    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // let mut v = vec![1, 3, 4];
    // for i in &mut v {
    //     *i += 50;
    // }
    //
    // for i in &v {
    //     println!("{i}");
    // }
    //

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.02),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}
