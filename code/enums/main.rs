// tuple structs
#[derive(Debug)]
struct Point2D(f64, f64);

#[derive(Debug)]
enum IpAddKind {
    V4,
    V6,
}

struct Ipv4Addr {}
struct Ipv6Addr {}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

#[derive(Debug, Copy, Clone)]
enum UsState {
    Massachusetts,
    California,
}

#[derive(Debug, Copy, Clone)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("Hello, Enums!");
    let origin = Point2D(0.0, 0.0);
    print!("o: {:#?}", origin);

    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    route(four);
    route(six);

    let m = Message::Write(String::from("hi"));
    m.call();

    // optionals
    let some_number = Some(11);
    let some_string = Some("indeed");
    let no_number: Option<i32> = None;

    // use if let when only 1 case is needed and other ones are ignored
    let new_num = plus_one(some_number);
    if let Some(i) = new_num {
        println!("new_num: {}", i)
    } else {
        println!("no number");
    }

    // match
    let quarter = Coin::Quarter(UsState::Massachusetts);
    println!("cents: {}", value_in_cents(quarter));

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let coins: [Coin; 5] = [
        Coin::Penny,
        Coin::Dime,
        Coin::Nickel,
        Coin::Quarter(UsState::Massachusetts),
        Coin::Quarter(UsState::California),
    ];

    let mut amount = 0u8;
    for coin in coins.iter() {
        if let Coin::Dime = coin {
            println!("A dime!");
        }
        amount += value_in_cents(*coin);
    }
    println!("total: {}", amount);
}

fn route(ip_kind: IpAddKind) {
    println!("{:#?}", ip_kind);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
