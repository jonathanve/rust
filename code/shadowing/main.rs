fn main() {
    const NUMBER: u32 = 100_000;
    println!("NUMBER: {}", NUMBER);

    // immutable
    let x = 1;
    println!("x: {}", x);

    // mutable
    let mut y = 2;
    println!("y: {}", y);
    y = 3;
    println!("y: {}", y);

    // shadowing (reuse same var name)
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    // byte and overflow
    let bt: u8 = b'A';
    println!("bt: {}", bt);

    let decimal: f64 = 9.11;
    println!("decimal: {}", decimal);

    let flag: bool = true;
    println!("flag: {}", flag);

    let heart_eyed_cat = '😻';
    println!("char: {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, b, _) = tup;
    println!("The value of b is: {}", b);

    let arr: [i32; 3] = [0, 9, 11];
    let idx = 1;
    println!("member: {}", arr[idx]);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("month: {}", months[8]);

    let row = [false; 3];
    let board = [row; 3];
    for r in &board {
        println!("{} {} {}", r[0], r[1], r[2])
    }

    let v = {
        let x = 3;
        x + 1
    };
    println!("v: {}", plus_one(v));

    let condition = true;
    let alpha: i32 = if condition { 2 } else { 0 };
    println!("alpha: {}", plus_one(alpha));
}

// expressions do not end with ;
// statements do not evaluate to a value
// if return is used, use ;
fn plus_one(x: i32) -> i32 {
    x + 1
}
