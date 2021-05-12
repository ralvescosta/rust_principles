fn main() {
    let v = Vec::<i32>::new();
    let mut v = vec![10,20,30,40,50];

    v.push(1);
    v.push(2);
    
    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2){
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }

    for i in &v {
        println!("{}", i);
    }
    println!("\n");
    let mut v= vec![10,20,30,40,50];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadSheetCell::Int(91),
        SpreadSheetCell::Text(String::from("green")),
        SpreadSheetCell::Float(98.11)
    ];
}
