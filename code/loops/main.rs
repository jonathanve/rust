use chrono::prelude::*;

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break 2 * counter;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for element in a.iter() {
        println!("fib({}) = {}", element, fib(*element));
    }

    for number in (1..5).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    let param = 50;
    let start = Local::now();
    let result = fib(param);
    let end = Local::now();
    println!("fib({}) = {} {}", param, result, end - start);
}

fn fib(n: u32) -> u64 {
    if n < 1 {
        return 0;
    }
    let mut x = 0;
    let mut y = 1;
    for _ in 1..n {
        let tmp = x + y;
        x = y;
        y = tmp;
    }
    y
}
