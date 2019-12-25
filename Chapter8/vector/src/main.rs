enum MultipleData {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    // let mut v = Vec::new();

    // v.push(5);
    // v.push(6);
    // println!("{:?}", v)
    let v = vec![1, 2, 3, 4, 5];

    let third: i32 = v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There's no third element."),
    }

    let mut v = vec![100, 200, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i)
    }

    let row = vec![
        MultipleData::Int(3),
        MultipleData::Text(String::from("Pink")),
        MultipleData::Float(3.14),
    ];
}
