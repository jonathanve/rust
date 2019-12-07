use std::collections::HashMap;

fn main() {
    println!("Vectors");

    let mut v1: Vec<f64> = Vec::new();
    v1.push(1.0911314);
    v1.push(-1.0);
    v1.push(9.1);
    for i in &v1 {
        println!("{}", i);
    }
    
    let v2 = vec!["hi", "mate"];
    match v2.get(1) {
        Some(text) => println!("{}", text),
        None => println!("no element"),
    }

    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let v3 = vec!([
        Cell::Int(9),
        Cell::Float(11.0),
        Cell::Text(String::from("vector")),
    ]);
    match v3.get(1) {
        Some(element) => {
            println!("{:#?}", element)
        },
        None => println!("no element"),
    }

    println!("Strings");

    let s1 = String::from("united states and colombia");
    let s2 = "(us, co)";
    let s3 = format!("{} {}", s1, s2);

    println!("chars");
    for c in s3.chars() {
        println!("{}", c);
    }
    println!("bytes");
    for b in s3.bytes() {
        println!("{}", b);
    }

    println!("Maps");

    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 1);
    scores.insert(String::from("Green"), 10);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        Some(score) => println!("{}", score),
        None => println!("no score"),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "love is the death of duty duty is the death of love";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
